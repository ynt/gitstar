use reqwest;
use reqwest::header::Headers;
use std::io;
use serde_json;
use url::Url;
use serde_json::Value;
use std::time::Duration;

use super::error::Error;

pub mod proxy_pool;

pub struct Response {
    pub body: String,
    pub header: Headers,
}

impl Response {
    pub fn to_json(&self) -> Result<Value, Error> {
        let v: Value = serde_json::from_slice(self.body.as_bytes())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(v)
    }
}

pub fn get(url: &str) -> Result<Response, Error> {
    let proxy = proxy_pool::get();
    let target = reqwest::Url::parse(&format!("http://{}", proxy))?;

    match request(url, target) {
        Ok(res) => Ok(res),
        Err(Error::ReqwestErr(_)) => {
            println!("{:?}", "reqwest err, contain timeout");
            Err(Error::new("string".to_owned()))
        }
        Err(error) => {
            println!("{:?}", error);
            Err(error)
        }
    }
}

fn request(url: &str, proxy: Url) -> Result<Response, Error> {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::custom(move |url| {
            if url.host_str() != Some("2pm.me") {
                Some(proxy.clone())
            } else {
                None
            }
        }))
        .timeout(Some(Duration::from_secs(1)))
        .build()?;

    info!("fetch url: {}", url);
    let mut resp = client.get(url).send()?;

    let header = resp.headers().clone();

    Ok(Response {
        body: resp.text()?,
        header,
    })
}
