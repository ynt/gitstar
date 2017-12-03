extern crate gitstar;

#[macro_use]
extern crate log;
extern crate env_logger;

// use gitstar::*;
use gitstar::repo::BaseInfo;


fn main() {
    env_logger::init().unwrap();

    let base_info = BaseInfo::new(0, "centos".to_owned(), "AltArch".to_owned());

    let mut page = base_info.get_repo_info().unwrap();
    while !page.is_end() {
        let result = page.fetch().unwrap();

        println!("{:?}", result);

        info!(target: "yak_events", "current data len: {}", result.len());
    }

    let mut page = base_info.get_user_repos().unwrap();

    while !page.is_end() {
        let result = page.fetch().unwrap();

        println!("{:?}", result);

        info!(target: "yak_events", "current data len: {}", result.len());
    }
}
