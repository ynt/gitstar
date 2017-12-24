use models::schema::*;
use std::time::SystemTime;
use diesel::prelude::*;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "search_languages"]
#[primary_key(id)]
pub struct NewSerchLanguage<'a> {
    pub id: Option<i32>,
    pub language: &'a str,
    pub status: bool,

    pub publish_at: SystemTime,
    pub update_at: SystemTime,
}

impl<'a> NewSerchLanguage<'a> {
    pub fn new(id: Option<i32>, language: &'a str, status: bool) -> Self {
        let now = SystemTime::now();
        Self {
            id: id,
            language: language,
            status: status,
            publish_at: now,
            update_at: now,
        }
    }
}

#[derive(Queryable)]
pub struct SearchLanguage {
    pub id: i32,
    pub language: String,
    pub status: bool,

    pub publish_at: SystemTime,
    pub update_at: SystemTime,
}

impl SearchLanguage {
    pub fn get_all_language(conn: &PgConnection) -> Vec<SearchLanguage> {
        use models::schema::search_languages::dsl::*;
        search_languages
            .filter(status.eq(true))
            .load::<SearchLanguage>(conn)
            .expect("error")
    }
}
