pub fn get_option_bool(value: Option<bool>) -> bool {
    match value {
        Some(res) => res,
        None => false,
    }
}

pub fn get_option_i64(value: Option<i64>) -> i64 {
    match value {
        Some(res) => res,
        None => 0,
    }
}

pub fn get_option_string(value: Option<&str>) -> String {
    match value {
        Some(res) => res.to_owned(),
        None => String::new(),
    }
}
