extern crate gitstar;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate scron;
extern crate chrono;

// use gitstar::*;
use gitstar::repo::BaseInfo;

fn main() {
    get_data();
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
