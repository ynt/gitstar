use diesel;
use models::schema::*;
use diesel::prelude::*;

#[derive(Insertable, Identifiable, Debug)]
#[table_name = "license"]
#[primary_key(id)]
pub struct NewLicense<'a> {
    pub id: Option<i32>,
    pub key: &'a str,
    pub name: &'a str,
    pub spdx_id: &'a str,
    pub url: &'a str,
}

impl<'a> NewLicense<'a> {
    pub fn new(id: Option<i32>, key: &'a str, name: &'a str, spdx_id: &'a str, url: &'a str) -> Self {
        Self {
            id,
            key,
            name,
            spdx_id,
            url,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct License {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
}

impl License {
    pub fn find_id(conn: &PgConnection, l_key: &str, l_name: &str, l_spdx_id: &str, l_url: &str) -> i32 {
        use models::schema::license::dsl::*;

        let res = license
            .filter(key.eq(l_key))
            .filter(name.eq(l_name))
            .filter(spdx_id.eq(l_spdx_id))
            .filter(url.eq(l_url))
            .load::<License>(conn)
            .expect("err");

        if res.len() > 0 {
            match res.get(0) {
                Some(expr) => expr.id,
                None => License::insert_and_get_id(conn, l_key, l_name, l_spdx_id, l_url),
            }
        } else {
            License::insert_and_get_id(conn, l_key, l_name, l_spdx_id, l_url)
        }
    }

    fn insert_and_get_id(conn: &PgConnection, key: &str, name: &str, spdx_id: &str, url: &str) -> i32 {
        info!("license not find, insert");

        let data = NewLicense::new(None, key, name, spdx_id, url);
        let result: License = diesel::insert_into(license::table)
            .values(&data)
            .get_result(conn)
            .expect("err insert license");

        result.id
    }
}
