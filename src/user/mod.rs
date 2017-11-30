use hyper::Error;
use serde_json::Value;
use super::client;

// https://api.github.com/users/gowins/repos
pub fn get(name: String) -> Result<Value, Error> {

    let mut url = String::from("https://api.github.com/users/");

    url.push_str(&name);
    url.push_str("/repos");

    Ok(client::get(url)?)
}
