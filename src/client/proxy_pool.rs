use rand::{thread_rng, Rng};
use reqwest;

static mut PROXY_POOL: [&str; 30] = [""; 30];

pub fn get() -> (usize, &'static str) {
    let proxy: &'static str = format!("{:?}", get_proxy());
    unsafe {
        let index = thread_rng().gen_range(0, PROXY_POOL.len() - 1);
        if PROXY_POOL[index] == "" {
            PROXY_POOL[index] = proxy;
        }

        (index, PROXY_POOL[index])
    }
}

pub fn del_proxy(index: usize) {
    unsafe {
        if index < PROXY_POOL.len() {
            PROXY_POOL[index] = ""
        }
    }
}

pub fn get_proxy() -> String {
    let mut res = reqwest::Client::builder()
        .build()
        .expect("err")
        .get("https://proxy.2pm.me/get")
        .send()
        .expect("err");

    res.text().expect("error")
}
