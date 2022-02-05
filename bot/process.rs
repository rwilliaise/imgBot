use crate::bot::BotData;

use crate::command::CommandRunArgs;
use err_context::AnyError;
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use reqwest::Response;
use serde_json::json;
use shared::CommandError;
use tokio::sync::RwLockReadGuard;

pub async fn basic_img_job(
    r: &RwLockReadGuard<'_, BotData>,
    a: &CommandRunArgs,
    request_url: &str,
) -> Result<Response, AnyError> {
    r.check_health().await?;

    let mut img_url: String;
    let url = a.matches.value_of("url");
    if url.is_none() {
        let url = r.latest_image.get(&a.msg.channel_id);

        match url {
            Some(url) => {
                img_url = url.clone();
            }
            None => {
                return Err(CommandError::GenericError(
                    "No url provided. Try sending a new image, or specify a url with -u.",
                ).into());
            }
        }
    } else {
        img_url = url.unwrap().to_string();
    }

    let request = json!({
        "target_url": img_url,
        "text": a.matches.values_of("text").ok_or(CommandError::GenericError("No caption provided"))?.collect::<Vec<&str>>().join(" ")
    });

    let out = r
        .construct_post(request_url)
        .await
        .json(&request)
        .send()
        .await;

    if let Err(e) = out {
        return Err(Box::new(e));
    }

    return Ok(out.unwrap());
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
