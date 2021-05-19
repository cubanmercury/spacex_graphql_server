#[macro_use]
extern crate diesel;
extern  crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod database;

use crate::database::models::*;
use crate::database::schema::*;


pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL is not set");

  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}