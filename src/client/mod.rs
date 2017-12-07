use reqwest;
use std::io;
use serde_json;
use serde_json::Value;

use super::error::Error;

pub struct Response {
    pub body: String,
}

impl Response {
    pub fn to_json(&self) -> Result<Value, Error> {
        let v: Value = serde_json::from_slice(self.body.as_bytes()).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        })?;
        Ok(v)
    }
}

pub fn get(url: &str) -> Result<Response, Error> {
    let target = reqwest::Url::parse("http://127.0.0.1:8001")?;
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::custom(
            move |url| if url.host_str() != Some("hyper.rs") {
                Some(target.clone())
            } else {
                None
            },
        ))
        .build()?;

    info!("fetch url: {}", url);
    let mut resp = client.get(url).send()?;

    Ok(Response { body: resp.text()? })
}
