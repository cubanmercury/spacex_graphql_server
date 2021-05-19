extern crate diesel;
extern crate spacex_graphql_rust;

use self::spacex_graphql_rust::*;
use self::database::models::*;
use diesel::prelude::*;




fn main() {
  use self::database::schema::roadster_info::dsl::*;


  let connection = establish_connection();
  let results = roadster_info.filter(norad_id.eq(43205))
    .limit(5)
    .load::<Roadster>(&connection)
    .expect("Error loading Roadster info");

  println!("Displaying {} roadster lines", results.len());
  for info in results {
    match info.name {
      Some(n) => println!("{}", n),
      None => println!("info name not found")
    };
    println!("------------\n");
    match info.details {
      Some(d) => println!("details: {}", d),
      None => println!("details not found")
    }
  }
}