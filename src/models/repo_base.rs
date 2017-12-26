use models::schema::*;
use std::time::SystemTime;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "repo_base"]
#[primary_key(id)]
pub struct NewRepoBase<'a> {
    pub id: i64,
    pub license_id: i64,
    pub owner_id: i64,
    pub name: &'a str,
    pub full_name: &'a str,
    pub private: bool,
    pub html_url: &'a str,
    pub description: &'a str,
    pub create_at: SystemTime,
    pub homepage: &'a str,
    pub lanauage: &'a str,
    pub insert_time: SystemTime,
}

#[derive(Queryable)]
pub struct RepoBase {
    pub id: i64,
    pub license_id: i64,
    pub owner_id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub html_url: String,
    pub description: String,
    pub create_at: SystemTime,
    pub homepage: String,
    pub lanauage: String,
    pub insert_time: SystemTime,
}
