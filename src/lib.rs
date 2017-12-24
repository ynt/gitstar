#![recursion_limit = "128"]
// logic
pub mod client;

pub mod page;

pub mod repo;

pub mod prelude;

// db
pub mod models;

// util
pub mod util;

pub mod error;

pub mod app;

// use cate
extern crate reqwest;

extern crate serde_json;

#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate diesel;
// extern crate dotenv;

extern crate diesel_infer_schema;

extern crate url;

extern crate regex;
