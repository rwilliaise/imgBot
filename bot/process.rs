use crate::command::CommandRunArgs;
use err_context::AnyError;
use reqwest::header::CONTENT_TYPE;
use reqwest::Response;
use serde_json::json;
use serenity::http::{AttachmentType, Http};
use serenity::model::channel::{Message, MessageReference};
use shared::CommandError;
use std::borrow::{Borrow, Cow};
use std::sync::Arc;
use std::time::Duration;
use linkify::LinkFinder;
use crate::bot::BotLock;

pub async fn get_first_attachment(message: &Message) -> Option<String> {
    for attachment in &message.attachments {
        if let Some(content) = &attachment.content_type {
            if content.starts_with("image") {
                return Some(attachment.url.clone());
            }
        }
    }

    None
}

pub async fn get_first_url(content: &String) -> Option<String> {
    let finder = LinkFinder::new();
    let links: Vec<_> = finder.links(content).collect();

    for link in links {
        let string = link.as_str().to_string().clone();
        if string.ends_with(".gif") || string.ends_with(".png") || string.ends_with(".jpg")
        {
            return Some(string)
        } else {
            let url = url::Url::parse(link.as_str());

            if let Ok(url) = url {
                if url.host_str() == Some("tenor.com") {
                    return Some(string)
                }
            }
        }
    }

    None
}

async fn url_from_reply(a: &CommandRunArgs) -> Option<String> {
    let reference = &a.msg.message_reference;

    if let None = reference {
        return None
    }

    let reply = reference.as_ref().unwrap();

    if reply.channel_id == a.msg.channel_id {
        let msg = a.http.get_message(reply.channel_id.0, reply.message_id.unwrap().0).await;

        if let Err(e) = msg {
            return None
        }

        let msg = msg.as_ref().unwrap();
        let url = get_first_url(&msg.content).await;

        if let None = url {
            if let Some(url) = get_first_attachment(msg).await {
                return Some(url);
            }

            return None
        }

        return Some(url.unwrap());
    }

    None
}

async fn generic_img_job(a: &CommandRunArgs, request_url: &str) -> Result<Response, AnyError> {
    let mut r = a.bot.write().await;

    r.check_health().await?;

    let mut img_url: String;
    let url = a.matches.value_of("url");
    if url.is_none() {

        if let Some(url) = url_from_reply(a).await {
            img_url = url.clone();
        } else {
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
        }
    } else {
        img_url = url.unwrap().to_string();
    }

    let url = url::Url::parse(img_url.as_str());
    if let Ok(url) = url {
        if url.host_str() == Some("tenor.com") {
            let gif = r.tenor_client.fetch(img_url).await?;
            img_url = gif.url;
        }
    }

    let request = json!({
        "target_url": img_url,
        "text": a.matches.values_of("text").ok_or(CommandError::GenericError("No text provided"))?.collect::<Vec<&str>>().join(" ")
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

async fn exploitable_img_job(
    a: &CommandRunArgs,
    request_url: &str,
) -> Result<Response, AnyError> {
    let r = a.bot.write().await;

    r.check_health().await?;

    let request = json!({
        "text": a.matches.values_of("text").ok_or(CommandError::GenericError("No text provided"))?.collect::<Vec<&str>>().join(" ")
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

pub async fn img_job(
    a: CommandRunArgs,
    request_url: &str,
    exploitable: bool,
) -> Result<(), AnyError> {
    let mut msg = a
        .msg
        .channel_id
        .say(a.http.clone(), "1/3 🟩⬛⬛ Requesting")
        .await?;

    let response = match exploitable {
        true => exploitable_img_job(&a, request_url).await,
        false => generic_img_job(&a, request_url).await,
    };

    msg.edit(a.http.clone(), |m| m.content("2/3 🟩🟩⬛ Processing"))
        .await?;

    if let Err(e) = response {
        crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;
        return Err(e);
    }

    let response = response?;

    match response.error_for_status_ref() {
        Ok(_) => (),
        Err(_) => {
            crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;
            let text = response.text().await?;
            return Err(CommandError::StringError(format!(
                "Image server contact failure.\n\n{}",
                text.as_str()
            ))
            .into());
        }
    }
    let mime = response
        .headers()
        .get(CONTENT_TYPE)
        .ok_or(CommandError::GenericError("Unknown format"))?;
    let is_gif = mime == "image/gif";

    let bytes = response.bytes().await?;

    msg.edit(a.http.clone(), |m| m.content("3/3 🟩🟩🟩 Uploading"))
        .await?;

    a.http
        .send_files(
            a.msg.channel_id.clone().into(),
            [AttachmentType::Bytes {
                data: Cow::Borrowed(bytes.borrow()),
                filename: match is_gif {
                    // TODO: support more content types
                    true => format!("{}.gif", a.name.clone()),
                    false => format!("{}.png", a.name.clone()),
                },
            }],
            serde_json::Map::default(),
        )
        .await?;

    crate::process::delay_delete(a.http.clone(), msg, Duration::from_millis(1000)).await;

    Ok(())
}

pub async fn delay_delete(http: Arc<Http>, msg: Message, duration: Duration) {
    tokio::spawn(async move {
        tokio::time::sleep(duration).await;
        msg.delete(http.clone()).await.unwrap();
    });
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
