use serde_json::Value;
use super::{user, client};
use super::error::Error;

// https://api.github.com/repos/google/gops
pub fn get(user_info: user::UserInfo) -> Result<Value, Error> {

    let mut url = String::from("https://api.github.com/repos/");

    url.push_str(&user_info.get_full_name());

    Ok(client::get(&url)?.to_json()?)
}
