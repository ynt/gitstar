pub mod schema;

pub mod owner;

pub mod search_language;

pub mod license;

pub mod repo_base;

use diesel::prelude::*;

pub struct Connect {
    pub db: PgConnection,
}

use std::fmt::Debug;

use diesel;
use diesel::pg::Pg;
use diesel::insertable::{InsertValues, CanInsertInSingleQuery};
use diesel::query_builder::QueryFragment;
impl Connect {
    pub fn new(dsn: &str) -> Self {
        Connect { db: PgConnection::establish(dsn).expect(&format!("Error connecting to {}", dsn)) }
    }

    pub fn print<T>(data: &T)
    where
        T: Debug,
    {
        println!("{:?}", data);
    }

    pub fn save<T, U>(&self, target: T, data: U)
    where
        T: Table,
        T::FromClause: QueryFragment<Pg>,
        U: Insertable<T>,
        U::Values: InsertValues<T, Pg> + CanInsertInSingleQuery<Pg>,
    {
        diesel::insert_into(target)
            .values(data)
            .execute(&self.db)
            .expect("Error");
    }
}
