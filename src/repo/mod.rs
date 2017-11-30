use hyper::Error;
use serde_json::Value;
use super::client;

// https://api.github.com/repos/google/gops
pub fn get(full_name: String) -> Result<Value, Error> {

    let mut url = String::from("https://api.github.com/repos/");

    url.push_str(&full_name);

    Ok(client::get(url)?)
}
