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

fn main() {
    env_logger::init().unwrap();

    let _url1 = "https://api.github.com/repos/google/gops";
    let _url2 = "https://api.github.com/search/repositories?q=language:go&sort=stars&order=desc";

    let mut _page = Page2::new(_url2, 0, 100, true);

    // while !_page.is_end() {
    // _page.fetch();
    // println!("{:?}", _page.fetch());
    println!("{:?}", _page.get_url_list());
    // }



    // println!("{:?}", res.unwrap().to_json());
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });

    // thread::spawn(move || {
    //     let received = rx.recv().unwrap();
    //     println!("Got: {}", received);
    // });


    // let handle1 = thread::spawn(|| for i in 1..20 {
    //     println!("{:?}", i);
    // });

    // let handle2 = thread::spawn(|| for i in 20..30 {
    //     println!("{:?}", i);
    // });
    // handle1.join().unwrap();

    // handle2.join().unwrap();
    // get_data();
    // use scron::Schedule;
    // use chrono::Utc;
    // use std::str::FromStr;
    // let expression = "0/2 * * * * *";
    // let schedule = Schedule::from_str(expression).unwrap();
    // println!("Upcoming fire times:");
    // for datetime in schedule.upcoming(Utc).take(20) {
    //     println!("-> {}", datetime);
    // }
    // gitstar::models::create();
    // gitstar::models::delete();
    // gitstar::models::update();
    // gitstar::models::read();
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
