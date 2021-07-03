use serde_derive::Deserialize;
use chrono::{DateTime, Utc};

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
#[derive(Insertable, AsChangeset)]
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
  pub row_updated: &'a DateTime<Utc>,
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
#[derive(Insertable, AsChangeset)]
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
  pub row_updated: &'a DateTime<Utc>,
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
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable)]
pub struct Cores {
  pub id: String,
  pub block: Option<i32>,
  pub reuse_count: Option<i32>,
  pub rtls_attempts: Option<i32>,
  pub rtls_landings: Option<i32>,
  pub asds_attempts: Option<i32>,
  pub asds_landings: Option<i32>,
  pub last_update: Option<String>,
  pub launches: Option<Vec<String>>,
  pub serial: Option<String>,
  pub status: Option<String>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="cores"]
pub struct UpdateCores<'a> {
  pub id: &'a str,
  pub block: &'a i32,
  pub reuse_count: &'a i32,
  pub rtls_attempts: &'a i32,
  pub rtls_landings: &'a i32,
  pub asds_attempts: &'a i32,
  pub asds_landings: &'a i32,
  pub last_update: &'a String,
  pub launches: &'a Vec<String>,
  pub serial: &'a String,
  pub status: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable)]
pub struct Crew {
  pub id: String,
  pub name: Option<String>,
  pub agency: Option<String>,
  pub image: Option<String>,
  pub wikipedia: Option<String>,
  pub launches: Option<Vec<String>>,
  pub status: Option<String>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="crew"]
pub struct UpdateCrew<'a> {
  pub id: &'a str,
  pub name: &'a String,
  pub agency: &'a String,
  pub image: &'a String,
  pub wikipedia: &'a String,
  pub launches: &'a Vec<String>,
  pub status: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable)]
pub struct HeatShield {
  material: Option<String>,
  size_meters: Option<f64>,
  temp_degrees: Option<i32>,
  dev_partner: Option<String>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Mass {
  pub kg: Option<i32>,
  pub lb: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Volume {
  pub cubic_meters: Option<i32>,
  pub cubic_feet: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Distance {
  pub meters: Option<f64>,
  pub feet: Option<f64>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct PressurizedCapsule {
  pub payload_volume: Volume,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Cargo {
  pub solar_array: Option<i32>,
  pub unpressurized_cargo: bool,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Trunk {
  pub trunk_volume: Volume,
  pub cargo: Cargo,
}
#[derive(Debug, Deserialize, Queryable)]
#[allow(non_snake_case)]
pub struct Thrust {
  pub kN: Option<f64>,
  pub lbf: Option<f64>,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Thrusters {
  pub r#type: Option<String>,
  pub amount: Option<i32>,
  pub pods: Option<i32>,
  pub fuel_1: Option<String>,
  pub fuel_2: Option<String>,
  pub isp: Option<i32>,
  pub thrust: Thrust,
}
#[derive(Debug, Deserialize, Queryable)]
pub struct Dragons {
  pub id: String,
  pub heat_shield: HeatShield,
  pub launch_payload_mass: Mass,
  pub launch_payload_vol: Volume,
  pub return_payload_mass: Mass,
  pub return_payload_vol: Volume,
  pub pressurized_capsule: PressurizedCapsule,
  pub trunk: Trunk,
  pub height_w_trunk: Distance,
  pub diameter: Distance,
  pub first_flight: Option<String>,
  pub flickr_images: Option<Vec<String>>,
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub active: bool,
  pub crew_capacity: Option<i32>,
  pub sidewall_angle_deg: Option<i32>,
  pub orbit_duration_yr: Option<i32>,
  pub dry_mass_kg: Option<i32>,
  pub dry_mass_lbs: Option<i32>,
  pub thrusters: Vec<Thrusters>,
  pub wikipedia: Option<String>,
  pub description: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="dragons_heat_shield"]
pub struct UpdateDragonsHeatShield<'a> {
  pub id: &'a i32,
  pub dragon_id: &'a String,
  pub material: &'a String,
  pub size_meters: &'a f64,
  pub temp_degrees: &'a i32,
  pub dev_partner: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="dragons_pressurized_capsule"]
pub struct UpdateDragonsPressurizedCapsule<'a> {
  pub id: &'a i32,
  pub dragon_id: &'a String,
  pub payload_volume_cubic_meters: &'a i32,
  pub payload_volume_cubic_feet: &'a i32,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="dragons_trunk"]
pub struct UpdateDragonsTrunk<'a> {
  pub id: &'a i32,
  pub dragon_id: &'a String,
  pub trunk_volume_cubic_meters: &'a i32,
  pub trunk_volume_cubic_feet: &'a i32,
  pub cargo_solar_array: &'a i32,
  pub cargo_unpressurized_cargo: &'a bool,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="dragons_thrusters"]
pub struct UpdateDragonsThrusters<'a> {
  pub id: &'a i32,
  pub dragon_id: &'a String,
  pub type_: &'a String,
  pub amount: &'a i32,
  pub pods: &'a i32,
  pub fuel_1: &'a String,
  pub fuel_2: &'a String,
  pub isp: &'a i32,
  pub thrust_kn: &'a f64,
  pub thrust_lbf: &'a f64,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="dragons"]
pub struct UpdateDragons<'a> {
  pub dragon_id: &'a str,
  pub launch_payload_mass_kg: &'a i32,
  pub launch_payload_mass_lbs: &'a i32,
  pub launch_payload_vol_cubic_meters: &'a i32,
  pub launch_payload_vol_cubic_feet: &'a i32,
  pub return_payload_mass_kg: &'a i32,
  pub return_payload_mass_lbs: &'a i32,
  pub return_payload_vol_cubic_meters: &'a i32,
  pub return_payload_vol_cubic_feet: &'a i32,
  pub height_w_trunk_meters: &'a f64,
  pub height_w_trunk_feet: &'a f64,
  pub diameter_meters: &'a f64,
  pub diameter_feet: &'a f64,
  pub first_flight: &'a String,
  pub flickr_images: &'a Vec<String>,
  pub name: &'a String,
  pub type_: &'a String,
  pub active: &'a bool,
  pub crew_capacity: &'a i32,
  pub sidewall_angle_deg: &'a i32,
  pub orbit_duration_yr: &'a i32,
  pub dry_mass_kg: &'a i32,
  pub dry_mass_lbs: &'a i32,
  pub wikipedia: &'a String,
  pub description: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}
