extern crate gitstar;

use gitstar::repo;
fn main() {
    let result = repo::get(String::from("gladmo/film-info")).unwrap();

    println!("{:?}", result["archive_url"]);
}
