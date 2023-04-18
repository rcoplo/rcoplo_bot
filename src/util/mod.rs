pub mod macros;

use base64::Engine as _;
use reqwest::header::HeaderMap;
use crate::error::{BotError, BotResult};


pub fn bool_from_bool_deserializer<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let i = u32::deserialize(deserializer)?;
    Ok(i != 0)
}



pub async fn http_get(url: &str) -> BotResult<String> {
    let data = reqwest::ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.164 Safari/537.36")
        .build()?
        .get(url)
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}
pub async fn http_get_image(url: &str) -> BotResult<String> {
    let bytes = match reqwest::get(url).await {
        Ok(res) => {
            res
        }
        Err(err) => {
            return Err(BotError::from(format!("获取图片失败,响应码: {:?}\n image url: {}", err.status(), url)));
        }
    };
    match tokio::time::timeout(std::time::Duration::from_secs(60), bytes.bytes()).await {
        Ok(bytes) => {
            match bytes {
                Ok(b) => {
                    Ok(base64::engine::general_purpose::STANDARD.encode(b))
                }
                Err(err) => {
                    return return Err(BotError::from(format!("获取图片失败,响应码: {:?}\n image url: {}", err.status(), url)));
                }
            }
        },
        Err(_) => {
            Err(BotError::from("获取图片超时喵..."))
        }
    }
}
pub async fn http_post_json(url: &str, json: &serde_json::Value) -> BotResult<String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let data = client
        .post(url)
        .headers(headers)
        .json(json)
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}
