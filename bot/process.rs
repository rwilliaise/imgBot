use std::sync::Arc;
use std::time::Duration;
use crate::command::CommandRunArgs;
use err_context::AnyError;
use reqwest::Response;
use serde_json::json;
use serenity::http::Http;
use serenity::model::channel::Message;
use shared::CommandError;

pub async fn basic_img_job(
    a: &CommandRunArgs,
    request_url: &str,
) -> Result<Response, AnyError> {
    let mut r = a.bot.write().await;

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
                )
                .into());
            }
        }
    } else {
        img_url = url.unwrap().to_string();
    }

    let url = url::Url::parse(img_url.as_str());
    if let Ok(url) = url {
        if url.host_str() == Some("tenor.com") {
            let gif = r
                .tenor_client
                .fetch(img_url)
                .await?;
            img_url = gif.url;
        }
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

pub async fn delay_delete(http: Arc<Http>, msg: Message, duration: Duration) {
    tokio::spawn(async move {
        tokio::time::sleep(duration).await;
        msg.delete(http.clone()).await.unwrap();
    }).await.expect("delayed delete failure");
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
