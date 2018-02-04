use models::schema::*;
use diesel::prelude::*;
use std::time::SystemTime;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "owners"]
#[primary_key(id)]
pub struct NewOwner<'a> {
    pub id: i64,
    pub login: &'a str,
    pub avatar_url: &'a str,
    pub gravatar_id: &'a str,
    pub url: &'a str,
    pub html_url: &'a str,
    pub user_type: &'a str,
    pub site_admin: bool,

    pub publish_at: SystemTime,
    pub update_at: SystemTime,
}

impl<'a> NewOwner<'a> {
    pub fn new(
        id: i64,
        login: &'a str,
        avatar_url: &'a str,
        gravatar_id: &'a str,
        url: &'a str,
        html_url: &'a str,
        user_type: &'a str,
        site_admin: bool,
    ) -> Self {
        let now = SystemTime::now();
        NewOwner {
            id,
            login,
            avatar_url,
            gravatar_id,
            url,
            html_url,
            user_type,
            site_admin,
            publish_at: now,
            update_at: now,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct Owner {
    pub id: i64,
    pub login: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub user_type: String,
    pub site_admin: bool,

    pub publish_at: SystemTime,
    pub update_at: SystemTime,
}

pub fn find_owner_by_id(conn: &PgConnection, o_id: i64) -> Vec<Owner> {
    use models::schema::owners::dsl::*;

    let results = owners
        .filter(id.eq(o_id))
        .load::<Owner>(conn)
        .expect("Error loading Owner");

    results
}

pub fn read_and_output(db_connection: &PgConnection) {
    use models::schema::owners::dsl::*;
    use diesel::prelude::*;

    let results = owners
        .limit(5)
        .load::<Owner>(db_connection)
        .expect("Error loading Owner");

    println!("Returned results: {}", results.len());

    for r in results {
        println!("{:?}", r);
    }
}
