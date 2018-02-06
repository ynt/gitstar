use reqwest;
use reqwest::header::Headers;
use std::io;
use serde_json;
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

pub fn get(url: &str) -> Response {
    let resp = request(url);
    if let Err(Error::ReqwestErr(e)) = resp {
        match e.get_ref().and_then(|e| e.downcast_ref::<io::Error>()) {
            Some(err) => {
                println!("{}, this is err line", err);
                return get(url);
            }
            _ => {
                println!("{:?}", "what?!");
                Response {
                    body: "".to_owned(),
                    header: reqwest::header::Headers::new(),
                }
            }
        }
    } else {
        match resp {
            Ok(result) => result,
            Err(e) => {
                println!("{:?}", e);
                Response {
                    body: "".to_owned(),
                    header: reqwest::header::Headers::new(),
                }
            }
        }
    }
}

fn request(url: &str) -> Result<Response, Error> {
    let proxy = proxy_pool::get();
    let target = reqwest::Url::parse(&format!("http://{}", proxy))?;

    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::custom(move |url| {
            if url.host_str() != Some("2pm.me") {
                Some(target.clone())
            } else {
                None
            }
        }))
        .timeout(Some(Duration::from_secs(8)))
        .build()?;

    info!("fetch url: {}", url);
    let mut resp = client.get(url).send()?;

    let header = resp.headers().clone();

    println!("{:?}", resp.status());
    // put the proxy to the poll
    proxy_pool::put(proxy);
    Ok(Response {
        body: resp.text()?,
        header,
    })
}
