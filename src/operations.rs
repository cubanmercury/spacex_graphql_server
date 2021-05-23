use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;


use crate::database::models::*;
use crate::database::schema::*;

use {Roadster, UpdateRoadster};


pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL is not set");

  PgConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

// Method to add roadster_info to database table['roadster_info']
pub fn create_roadster<'a>(
  conn: &PgConnection,
  roadster_details: &'a Roadster
) -> Roadster {

  let new_roadster_entry = UpdateRoadster {
    apoapsis_au: &roadster_details.apoapsis_au.unwrap(),
    details: &roadster_details.details.as_ref().unwrap(),
    earth_distance_mi: &roadster_details.earth_distance_mi.unwrap(),
    earth_distance_km: &roadster_details.earth_distance_km.unwrap(),
    eccentricity: &roadster_details.eccentricity.unwrap(),
    epoch_jd: &roadster_details.epoch_jd.unwrap(),
    id: &roadster_details.id,
    inclination: &roadster_details.inclination.unwrap(),
    launch_date_unix: &roadster_details.launch_date_unix.unwrap(),
    launch_date_utc: &roadster_details.launch_date_utc.as_ref().unwrap(),
    launch_mass_kg: &roadster_details.launch_mass_kg.unwrap(),
    launch_mass_lbs: &roadster_details.launch_mass_lbs.unwrap(),
    longitude: &roadster_details.longitude.unwrap(),
    mars_distance_km: &roadster_details.mars_distance_km.unwrap(),
    mars_distance_mi: &roadster_details.mars_distance_mi.unwrap(),
    name: &roadster_details.name.as_ref().unwrap(),
    norad_id: &roadster_details.norad_id.unwrap(),
    orbit_type: &roadster_details.orbit_type.as_ref().unwrap(),
    periapsis_arg: &roadster_details.periapsis_arg.unwrap(),
    periapsis_au: &roadster_details.periapsis_au.unwrap(),
    period_days: &roadster_details.period_days.unwrap(),
    semi_major_axis_au: &roadster_details.semi_major_axis_au.unwrap(),
    speed_kph: &roadster_details.speed_kph.unwrap(),
    speed_mph: &roadster_details.speed_mph.unwrap(),
    video: &roadster_details.video.as_ref().unwrap(),
    wikipedia: &roadster_details.wikipedia.as_ref().unwrap(),
    flickr_images: &roadster_details.flickr_images.as_ref().unwrap()
  };

  diesel::insert_into(roadster_info::table)
    .values(&new_roadster_entry)
    .get_result(conn)
    .expect("Error saving entry to Roadster_info!")
}

// Method to remove roadster row from roadster_info table by ID
pub fn delete_roadster_by_id<'a>(
  conn: &PgConnection,
  roadster_details: &'a Roadster
) {
  use roadster_info::dsl::{roadster_info};

  let roadster_id = &roadster_details.id;

  let roadster_to_delete = diesel::delete(roadster_info.find(roadster_id))
    .execute(conn)
    .expect("Unable to delete Roadster entry");

  println!("Deleted {} roadster entry", roadster_to_delete);
}

// Method to add company_info to database table['company_info']
pub fn create_company<'a>(
  conn: &PgConnection,
  company_details: &'a Company
) {
  let new_company_entry = UpdateCompany {
    id: &company_details.id,
    name: &company_details.name.as_ref().unwrap(),
    founder: &company_details.founder.as_ref().unwrap(),
    founded: &company_details.founded.unwrap(),
    employees: &company_details.employees.unwrap(),
    vehicles: &company_details.vehicles.unwrap(),
    launch_sites: &company_details.launch_sites.unwrap(),
    test_sites: &company_details.test_sites.unwrap(),
    ceo: &company_details.ceo.as_ref().unwrap(),
    cto: &company_details.cto.as_ref().unwrap(),
    coo: &company_details.coo.as_ref().unwrap(),
    cto_propulsion: &company_details.cto_propulsion.as_ref().unwrap(),
    valuation: &company_details.valuation.unwrap(),
    summary: &company_details.summary.as_ref().unwrap(),
    headquarters_address: &company_details.headquarters.address.as_ref().unwrap(),
    headquarters_city: &company_details.headquarters.city.as_ref().unwrap(),
    headquarters_state: &company_details.headquarters.state.as_ref().unwrap(),
    links_website: &company_details.links.website.as_ref().unwrap(),
    links_flickr: &company_details.links.flickr.as_ref().unwrap(),
    links_twitter: &company_details.links.twitter.as_ref().unwrap(),
    links_elon_twitter: &company_details.links.elon_twitter.as_ref().unwrap(),
  };

  diesel::insert_into(company_info::table)
    .values(&new_company_entry)
    .execute(conn)
    .expect("Error saving entry to company_info");
}
// Method to remove company row from company_info table by ID
pub fn delete_company_by_id<'a>(
  conn: &PgConnection,
  company_details: &'a Company
) {
  use company_info::dsl::{company_info};
  let company_row_id = &company_details.id;

  let company_to_delete = diesel::delete(company_info.find(company_row_id))
    .execute(conn)
    .expect("Unable to delete Company entry");
  
  println!("Deleted {} Company entry", company_to_delete);
}