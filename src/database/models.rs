use serde_derive::Deserialize;

// use super::schema::roadster_info;
// use super::schema::company_info;
use super::schema::*;

#[derive(Queryable, Deserialize, Debug)]
pub struct Roadster {
  pub apoapsis_au: Option<f64>,
  pub details: Option<String>,
  pub earth_distance_mi: Option<f64>,
  pub earth_distance_km: Option<f64>,
  pub eccentricity: Option<f64>,
  pub epoch_jd: Option<f64>,
  pub id: String,
  pub inclination: Option<f64>,
  pub launch_date_unix: Option<i32>,
  pub launch_date_utc: Option<String>,
  pub launch_mass_kg: Option<i32>,
  pub launch_mass_lbs: Option<i32>,
  pub longitude: Option<f64>,
  pub mars_distance_km: Option<f64>,
  pub mars_distance_mi: Option<f64>,
  pub name: Option<String>,
  pub norad_id: Option<i32>,
  pub orbit_type: Option<String>,
  pub periapsis_arg: Option<f64>,
  pub periapsis_au: Option<f64>,
  pub period_days: Option<f64>,
  pub semi_major_axis_au: Option<f64>,
  pub speed_kph: Option<f64>,
  pub speed_mph: Option<f64>,
  pub video: Option<String>,
  pub wikipedia: Option<String>,
  pub flickr_images: Option<Vec<String>>,
}

#[derive(Insertable)]
#[table_name="roadster_info"]
pub struct UpdateRoadster<'a> {
  pub apoapsis_au: &'a f64,
  pub details: &'a String,
  pub earth_distance_mi: &'a f64,
  pub earth_distance_km: &'a f64,
  pub eccentricity: &'a f64,
  pub epoch_jd: &'a f64,
  pub id: &'a str,
  pub inclination: &'a f64,
  pub launch_date_unix: &'a i32,
  pub launch_date_utc: &'a String,
  pub launch_mass_kg: &'a i32,
  pub launch_mass_lbs: &'a i32,
  pub longitude: &'a f64,
  pub mars_distance_km: &'a f64,
  pub mars_distance_mi: &'a f64,
  pub name: &'a String,
  pub norad_id: &'a i32,
  pub orbit_type: &'a String,
  pub periapsis_arg: &'a f64,
  pub periapsis_au: &'a f64,
  pub period_days: &'a f64,
  pub semi_major_axis_au: &'a f64,
  pub speed_kph: &'a f64,
  pub speed_mph: &'a f64,
  pub video: &'a String,
  pub wikipedia: &'a String,
  pub flickr_images: &'a Vec<String>,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct CompanyHeadquarters {
  pub address: Option<String>,
  pub city: Option<String>,
  pub state: Option<String>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct CompanyLinks {
  pub website: Option<String>,
  pub flickr: Option<String>,
  pub twitter: Option<String>,
  pub elon_twitter: Option<String>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Company {
  pub id: String,
  pub name: Option<String>,
  pub founder: Option<String>,
  pub founded: Option<i32>,
  pub employees: Option<i32>,
  pub vehicles: Option<i32>,
  pub launch_sites: Option<i32>,
  pub test_sites: Option<i32>,
  pub ceo: Option<String>,
  pub cto: Option<String>,
  pub coo: Option<String>,
  pub cto_propulsion: Option<String>,
  pub valuation: Option<i64>,
  pub summary: Option<String>,
  pub headquarters: CompanyHeadquarters,
  pub links: CompanyLinks,
}

#[derive(Insertable)]
#[table_name="company_info"]
pub struct UpdateCompany<'a> {
  pub id: &'a str,
  pub name: &'a String,
  pub founder: &'a String,
  pub founded: &'a i32,
  pub employees: &'a i32,
  pub vehicles: &'a i32,
  pub launch_sites: &'a i32,
  pub test_sites: &'a i32,
  pub ceo: &'a String,
  pub cto: &'a String,
  pub coo: &'a String,
  pub cto_propulsion: &'a String,
  pub valuation: &'a i64,
  pub summary: &'a String,
  pub headquarters_address: &'a String,
  pub headquarters_city: &'a String,
  pub headquarters_state: &'a String,
  pub links_website: &'a String,
  pub links_flickr: &'a String,
  pub links_twitter: &'a String,
  pub links_elon_twitter: &'a String,
}

#[derive(Debug, Deserialize, Queryable)]
pub struct Capsules {
  pub id: String,
  pub reuse_count: Option<i32>,
  pub water_landings: Option<i32>,
  pub land_landings: Option<i32>,
  pub last_update: Option<String>,
  pub launches: Option<Vec<String>>,
  pub serial: Option<String>,
  pub status: Option<String>,
  pub r#type: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="capsules"]
pub struct UpdateCapsules<'a> {
  pub id: &'a str,
  pub reuse_count: &'a i32,
  pub water_landings: &'a i32,
  pub land_landings: &'a i32,
  pub last_update: &'a String,
  pub launches: &'a Vec<String>,
  pub serial: &'a String,
  pub status: &'a String,
  pub type_: &'a String,
}