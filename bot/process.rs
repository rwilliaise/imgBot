use crate::bot::BotData;

use err_context::AnyError;
use reqwest::header::{CONTENT_TYPE, HeaderMap};
use serde_json::json;
use reqwest::Response;
use tokio::sync::RwLockReadGuard;
use shared::CommandError;
use crate::command::CommandRunArgs;

pub async fn basic_img_job(r: &RwLockReadGuard<'_, BotData>, a: &CommandRunArgs, request_url: &str) -> Result<Response, AnyError> {
    r.check_health().await?;

    let url = r.latest_image.get(&a.msg.channel_id);

    let mut img_url: String;
    match url {
        Some(url) => {
            img_url = url.clone();
        }
        None => {
            let url = a.matches.value_of("url");
            if url.is_none() {
                return Err(CommandError::GenericError(
                    "No url provided. Try sending a new image, or specify a url with -u.",
                )
                    .into());
            }
            img_url = url.unwrap().to_string();
        }
    }

    let request = json!({
        "target_url": img_url,
        "text": a.matches.value_of("text").unwrap_or("No caption provided! :(")
    });

    let out = r.construct_post(request_url)
        .await
        .json(&request)
        .send()
        .await;

    if let Err(e) = out {
        return Err(Box::new(e))
    }

    return Ok(out.unwrap())
}

pub fn get_args(str: &String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let current: &mut Vec<char> = &mut Vec::new();
    let mut quote = false;

    for char in str.chars() {
        if char == ' ' && !quote && current.len() > 0 {
            out.push(String::from_iter(std::mem::take(current)));
            continue;
        }
        if char == '"' {
            if current.len() > 0 {
                out.push(String::from_iter(std::mem::take(current)));
            }
            quote = !quote;
            continue;
        }

        current.push(char);
    }

    if current.len() > 0 {
        out.push(String::from_iter(std::mem::take(current)));
    }

    out
}
