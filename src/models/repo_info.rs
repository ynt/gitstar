use models::schema::*;
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
    pub id: i64,
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
