extern crate gitstar;
extern crate regex;

extern crate env_logger;
extern crate log;

extern crate chrono;
extern crate scron;

use gitstar::prelude::*;

fn main() {
    let dsn = "postgres://postgres:password666@localhost/gitstar";
    let conn = Connect::new(dsn);
    env_logger::init();

    job::search_by_language(&conn);
}
