use models::schema::*;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "repo_infos"]
#[primary_key(id)]
pub struct NewRepoInfo<'a> {
    pub id: Option<i32>,
    pub base_id: i64,
    pub license_id: i32,
    pub owner_id: i64,
    pub insert_date: &'a str,
    pub size: i64,
    pub stars: i32,
    pub forks: i32,
    pub issues: i32,
    pub language: &'a str,
    pub updated_at: SystemTime,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub create_at: SystemTime,
}

#[derive(Queryable)]
pub struct RepoInfos {
    pub id: i32,
    pub base_id: i64,
    pub license_id: i32,
    pub owner_id: i64,
    pub insert_date: String,
    pub size: i64,
    pub stars: i32,
    pub forks: i32,
    pub issues: i32,
    pub language: String,
    pub updated_at: SystemTime,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub create_at: SystemTime,
}

pub fn repo_infos_is_insert(conn: &PgConnection, info_id: i64, insert: &str) -> bool {
    use models::schema::repo_infos::dsl::*;

    let res = repo_infos
        .filter(insert_date.eq(insert.to_owned()))
        .filter(base_id.eq(info_id))
        .load::<RepoInfos>(conn)
        .expect("ERROR");

    if res.len() == 0 {
        false
    } else {
        true
    }
}
