extern crate gitstar;

#[macro_use]
extern crate log;
extern crate env_logger;

// use gitstar::*;
use gitstar::user::Page;


fn main() {
    env_logger::init().unwrap();

    let mut page = Page::new("centos", 30);

    while !page.is_end() {
        let result = page.get().unwrap();

        // println!("{:?}", result);

        info!(target: "yak_events", "current data len: {}", result.len());
    }
}
