extern crate gitstar;
extern crate regex;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate scron;
extern crate chrono;

use gitstar::prelude::*;
use gitstar::repo::BaseInfo;
// use std::thread;
// use std::sync::mpsc;

fn main() {}

pub fn get_data2() {
    env_logger::init().unwrap();

    let _url1 = "https://api.github.com/repos/google/gops";
    let _url2 = "https://api.github.com/search/repositories?q=language:go&sort=stars&order=desc";

    let mut _page = Page2::new(_url1, 0, 0, false);

    // while !_page.is_end() {
    // _page.fetch();
    // println!("{:?}", _page.fetch());
    println!("{:?}", _page.get_url_list());
    // }
}

pub fn get_data() {
    env_logger::init().unwrap();

    let base_info = BaseInfo::new(0, "google".to_owned(), "".to_owned(), "go".to_owned());

    // let mut page = base_info.get_repo_by_search().unwrap();
    // while !page.is_end() {
    //     let result = page.fetch().unwrap();

    //     println!("{:?}", result);

    //     info!("current data len: {}", result.len());
    // }

    let mut page = base_info.get_user_repos().unwrap();

    while !page.is_end() {
        let result = page.fetch().unwrap();

        println!("{:?}", result);

        info!(target: "yak_events", "current data len: {}", result.len());
    }
}
