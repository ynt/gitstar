extern crate gitstar;
extern crate regex;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate scron;
extern crate chrono;

use gitstar::prelude::*;

use gitstar::repo::BaseInfo;

fn main() {
    let dsn = "postgres://postgres:password666@localhost/gitstar";
    let conn = Connect::new(dsn);
    env_logger::init();

    job::search_by_language(&conn);
}

pub fn get_data2() {
    let dsn = "postgres://postgres:password666@localhost/gitstar";
    let conn = Connect::new(dsn);
    env_logger::init();
    // let data = NewSerchLanguage::new(None, "go", true);
    // conn.save(search_languages::table, &data);
    // return;
    //
    let _url1 = "https://api.github.com/repos/gladmo/film-info";
    // let _url2 = "https://api.github.com/search/repositories?q=language:go&sort=stars&order=desc";
    // let mut _page = Page2::new(_url2, 0, 0, false);

    let mut _page = Page2::new(_url1, 0, 0, false);

    println!("{:?}", _page.get_url_list());
    let url_list = _page.get_url_list();


    if let Ok(res) = _page.get(&url_list[0]) {
        for one in &res {
            let owner = &one.owner;
            // exist, skip
            if owner::find_owner_by_id(&conn.db, owner.id).len() == 0 {
                let data = NewOwner::new(
                    owner.id,
                    &owner.login,
                    &owner.avatar_url,
                    &owner.gravatar_id,
                    &owner.url,
                    &owner.html_url,
                    &owner.user_type,
                    owner.site_admin,
                );
                conn.save(owners::table, &data);
            }
        }
    }
}

pub fn get_data() {
    env_logger::init();

    let base_info = BaseInfo::new(0, "google", "", "go");

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
