use models::schema::*;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "repo_base"]
#[primary_key(id)]
pub struct NewRepoBase<'a> {
    pub id: i64,
    pub license_id: i32,
    pub owner_id: i64,
    pub name: &'a str,
    pub full_name: &'a str,
    pub private: bool,
    pub html_url: &'a str,
    pub description: &'a str,
    pub create_at: SystemTime,
    pub homepage: &'a str,
    pub language: &'a str,
    pub insert_time: SystemTime,
}

#[derive(Queryable)]
pub struct RepoBase {
    pub id: i64,
    pub license_id: i32,
    pub owner_id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub create_at: SystemTime,
    pub homepage: String,
    pub language: String,
    pub insert_time: SystemTime,
}

pub fn find_repo_base_by_id(conn: &PgConnection, repo_id: i64) -> bool {
    use models::schema::repo_base::dsl::*;

    let res = repo_base
        .filter(id.eq(repo_id))
        .load::<RepoBase>(conn)
        .expect("Error loading RepoBase");

    if res.len() == 0 {
        false
    } else {
        true
    }
}
