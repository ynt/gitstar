use reqwest;

static mut PROXY_POOL: Option<Vec<String>> = None;
static MAX_POOL_SIZE: usize = 30;

pub fn snapshot() -> Vec<String> {
    unsafe {
        match PROXY_POOL {
            Some(ref mut proxy_vec) => {
                if proxy_vec.len() < MAX_POOL_SIZE / 2 {
                    cram_proxy()
                }
                proxy_vec.clone()
            }
            None => {
                PROXY_POOL = Some(gen_proxy());
                snapshot()
            }
        }
    }
}

pub fn get() -> String {
    unsafe {
        match PROXY_POOL {
            Some(ref mut proxy_vec) => {
                if proxy_vec.len() < MAX_POOL_SIZE / 2 {
                    cram_proxy()
                }
                proxy_vec.pop().unwrap()
            }
            None => {
                PROXY_POOL = Some(gen_proxy());
                get_one_proxy()
            }
        }
    }
}

pub fn put(proxy: String) {
    unsafe {
        match PROXY_POOL {
            Some(ref mut proxy_vec) => proxy_vec.push(proxy),
            None => {
                PROXY_POOL = Some(gen_proxy());
            }
        }
    }
}

fn cram_proxy() {
    unsafe {
        match PROXY_POOL {
            Some(ref mut proxy_vec) => {
                let curr_size = proxy_vec.len();
                for _ in 0..(MAX_POOL_SIZE - curr_size) {
                    proxy_vec.insert(0, get_one_proxy())
                }
            }
            None => {
                PROXY_POOL = Some(gen_proxy());
            }
        }
    }
}
fn gen_proxy() -> Vec<String> {
    let mut proxy_vec = Vec::new();
    for _ in 0..MAX_POOL_SIZE {
        proxy_vec.push(get_one_proxy());
    }
    proxy_vec
}

fn get_one_proxy() -> String {
    let mut res = reqwest::Client::builder()
        .build()
        .expect("err")
        .get("https://proxy.2pm.me/get")
        .send()
        .expect("err");

    res.text().expect("error")
}
