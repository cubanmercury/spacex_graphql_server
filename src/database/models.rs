use serde_derive::Deserialize;
use chrono::{DateTime, Utc};
use juniper::{FieldResult, GraphQLObject};

use super::schema::*;

#[derive(Queryable, Deserialize, Debug, GraphQLObject)]
#[graphql(description="Elon Musk's Telsa Roadster statistics & info")]
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


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct CompanyHeadquarters {
  pub address: Option<String>,
  pub city: Option<String>,
  pub state: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct CompanyLinks {
  pub website: Option<String>,
  pub flickr: Option<String>,
  pub twitter: Option<String>,
  pub elon_twitter: Option<String>,
}
#[derive(Debug, GraphQLObject)]
pub struct CompanyGraphQL {
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
  pub valuation: Option<f64>, // valuation data type difference (f64).  Juniper doesn't support i64 values
  pub summary: Option<String>,
  pub headquarters: CompanyHeadquarters,
  pub links: CompanyLinks,
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
  pub valuation: Option<i64>, // valuation data type difference (i64)
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
  pub valuation: &'a f64,
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


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
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


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
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


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
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


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct HeatShield {
  pub material: Option<String>,
  pub size_meters: Option<f64>,
  pub temp_degrees: Option<i32>,
  pub dev_partner: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Mass {
  pub kg: Option<i32>,
  pub lb: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Volume {
  pub cubic_meters: Option<i32>,
  pub cubic_feet: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Distance {
  pub meters: Option<f64>,
  pub feet: Option<f64>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct PressurizedCapsule {
  pub payload_volume: Volume,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Cargo {
  pub solar_array: Option<i32>,
  pub unpressurized_cargo: bool,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Trunk {
  pub trunk_volume: Volume,
  pub cargo: Cargo,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
#[allow(non_snake_case)]
pub struct Thrust {
  pub kN: Option<f64>,
  pub lbf: Option<f64>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Thrusters {
  pub r#type: Option<String>,
  pub amount: Option<i32>,
  pub pods: Option<i32>,
  pub fuel_1: Option<String>,
  pub fuel_2: Option<String>,
  pub isp: Option<i32>,
  pub thrust: Thrust,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
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
  pub dry_mass_lb: Option<i32>,
  pub thrusters: Vec<Thrusters>,
  pub wikipedia: Option<String>,
  pub description: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="dragons_heat_shield"]
pub struct UpdateDragonsHeatShield<'a> {
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
  pub dragon_id: &'a String,
  pub payload_volume_cubic_meters: &'a i32,
  pub payload_volume_cubic_feet: &'a i32,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="dragons_trunk"]
pub struct UpdateDragonsTrunk<'a> {
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
  pub id: &'a str,
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

#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct HistoryLinks {
  pub article: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct History {
  pub id: String,
  pub title: Option<String>,
  pub event_date_utc: Option<String>,
  pub event_date_unix: Option<i32>,
  pub details: Option<String>,
  pub links: HistoryLinks,
}

#[derive(Insertable, AsChangeset)]
#[table_name="history"]
pub struct UpdateHistory<'a> {
  pub id: &'a str,
  pub title: &'a String,
  pub event_date_utc: &'a String,
  pub event_date_unix: &'a i32,
  pub details: &'a String,
  pub links_article: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Landpads {
  pub id: String,
  pub name: Option<String>,
  pub full_name: Option<String>,
  pub status: Option<String>,
  pub r#type: Option<String>,
  pub locality: Option<String>,
  pub region: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub landing_attempts: Option<i32>,
  pub landing_successes: Option<i32>,
  pub wikipedia: Option<String>,
  pub details: Option<String>,
  pub launches: Option<Vec<String>>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="landpads"]
pub struct UpdateLandpads<'a> {
  pub id: &'a str,
  pub name: &'a String,
  pub full_name: &'a String,
  pub status: &'a String,
  pub type_: &'a String,
  pub locality: &'a String,
  pub region: &'a String,
  pub latitude: &'a f64,
  pub longitude: &'a f64,
  pub landing_attempts: &'a i32,
  pub landing_successes: &'a i32,
  pub wikipedia: &'a String,
  pub details: &'a String,
  pub launches: &'a Vec<String>,
  pub row_updated: &'a DateTime<Utc>,
}

#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchFairings {
  pub reused: Option<bool>,
  pub recovery_attempt: Option<bool>,
  pub recovered: Option<bool>,
  pub ships: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchLinkPatch {
  pub small: Option<String>,
  pub large: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchLinkReddit {
  pub campaign: Option<String>,
  pub launch: Option<String>,
  pub media: Option<String>,
  pub recovery: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchLinkFlickr {
  pub small: Option<Vec<String>>,
  pub original: Option<Vec<String>>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchLinks {
  pub patch: Option<LaunchLinkPatch>,
  pub reddit: Option<LaunchLinkReddit>,
  pub flickr: Option<LaunchLinkFlickr>,
  pub presskit: Option<String>,
  pub webcast: Option<String>,
  pub youtube_id: Option<String>,
  pub article: Option<String>,
  pub wikipedia: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchCores {
  pub core: Option<String>,
  pub flight: Option<i32>,
  pub gridfins: Option<bool>,
  pub legs: Option<bool>,
  pub reused: Option<bool>,
  pub landing_attempt: Option<bool>,
  pub landing_success: Option<bool>,
  pub landing_type: Option<String>,
  pub landpad: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchFailures {
  pub time: Option<i32>,
  pub altitude: Option<i32>,
  pub reason: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Launches {
  pub id: String,
  pub name: Option<String>,
  pub fairings: Option<LaunchFairings>,
  pub links: Option<LaunchLinks>,
  pub static_fire_date_utc: Option<String>,
  pub static_fire_date_unix: Option<i32>,
  pub tdb: Option<bool>,
  pub net: Option<bool>,
  pub window: Option<i32>,
  pub rocket: Option<String>,
  pub success: Option<bool>,
  pub failures: Option<Vec<LaunchFailures>>,
  pub details: Option<String>,
  pub crew: Option<Vec<String>>,
  pub ships: Option<Vec<String>>,
  pub capsules: Option<Vec<String>>,
  pub payloads: Option<Vec<String>>,
  pub launchpad: Option<String>,
  pub auto_update: Option<bool>,
  pub flight_number: Option<i32>,
  pub date_utc: Option<String>,
  pub date_unix: Option<i32>,
  pub date_local: Option<String>,
  pub date_precision: Option<String>,
  pub upcoming: Option<bool>,
  pub cores: Option<Vec<LaunchCores>>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="launches"]
pub struct UpdateLaunches<'a> {
  pub launch_id: &'a str,
  pub name: &'a String,
  pub static_fire_date_utc: &'a String,
  pub static_fire_date_unix: &'a i32,
  pub tdb: &'a bool,
  pub net: &'a bool,
  pub window_number: &'a i32,
  pub rocket: &'a String,
  pub success: &'a bool,
  pub details: &'a String,
  pub crew: &'a Vec<String>,
  pub ships: &'a Vec<String>,
  pub capsules: &'a Vec<String>,
  pub payloads: &'a Vec<String>,
  pub launchpad: &'a String,
  pub auto_update: &'a bool,
  pub flight_number: &'a i32,
  pub date_utc: &'a String,
  pub date_unix: &'a i32,
  pub date_local: &'a String,
  pub date_precision: &'a String,
  pub upcoming: &'a bool,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="launch_fairings"]
pub struct UpdateLaunchFairings<'a> {
  pub launch_id: &'a str,
  pub reused: &'a bool,
  pub recovery_attempt: &'a bool,
  pub recovered: &'a bool,
  pub ships: &'a Vec<String>,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="launch_links"]
pub struct UpdateLaunchLinks<'a> {
  pub launch_id: &'a str,
  pub patch_small: &'a String,
  pub patch_large: &'a String,
  pub reddit_campaign: &'a String,
  pub reddit_launch: &'a String,
  pub reddit_media: &'a String,
  pub reddit_recovery: &'a String,
  pub flickr_small: &'a Vec<String>,
  pub flickr_original: &'a Vec<String>,
  pub presskit: &'a String,
  pub webcast: &'a String,
  pub youtube_id: &'a String,
  pub article: &'a String,
  pub wikipedia: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="launch_cores"]
pub struct UpdateLaunchCores<'a> {
  pub id: &'a str,
  pub launch_id: &'a str,
  pub core: &'a String,
  pub flight: &'a i32,
  pub gridfins: &'a bool,
  pub legs: &'a bool,
  pub reused: &'a bool,
  pub landing_attempt: &'a bool,
  pub landing_success: &'a bool,
  pub landing_type: &'a String,
  pub landpad: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="launch_failures"]
pub struct UpdateLaunchFailures<'a> {
  pub id: &'a str,
  pub launch_id: &'a str,
  pub time: &'a i32,
  pub altitude: &'a i32,
  pub reason: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct LaunchPads {
  pub id: String,
  pub name: Option<String>,
  pub full_name: Option<String>,
  pub locality: Option<String>,
  pub region: Option<String>,
  pub timezone: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub launch_attempts: Option<i32>,
  pub launch_successes: Option<i32>,
  pub rockets: Option<Vec<String>>,
  pub launches: Option<Vec<String>>,
  pub status: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="launchpads"]
pub struct UpdateLaunchpads<'a> {
  pub id: &'a str,
  pub name: &'a String,
  pub full_name: &'a String,
  pub locality: &'a String,
  pub region: &'a String,
  pub timezone: &'a String,
  pub latitude: &'a f64,
  pub longitude: &'a f64,
  pub launch_attempts: &'a i32,
  pub launch_successes: &'a i32,
  pub rockets: &'a Vec<String>,
  pub launches: &'a Vec<String>,
  pub status: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct PayloadDragon {
  pub capsule: Option<String>,
  pub mass_returned_kg: Option<f64>,
  pub mass_returned_lbs: Option<f64>,
  pub flight_time_sec: Option<i32>,
  pub manifest: Option<String>,
  pub water_landing: Option<bool>,
  pub land_landing: Option<bool>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Payloads {
  pub id: String,
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub dragon: PayloadDragon,
  pub reused: Option<bool>,
  pub launch: Option<String>,
  pub customers: Option<Vec<String>>,
  pub norad_ids: Option<Vec<i32>>,
  pub nationalities: Option<Vec<String>>,
  pub manufacturers: Option<Vec<String>>,
  pub mass_kg: Option<f64>,
  pub mass_lbs: Option<f64>,
  pub orbit: Option<String>,
  pub reference_system: Option<String>,
  pub regime: Option<String>,
  pub longitude: Option<f64>,
  pub semi_major_axis_km: Option<f64>,
  pub eccentricity: Option<f64>,
  pub periapsis_km: Option<f64>,
  pub apoapsis_km: Option<f64>,
  pub inclination_deg: Option<f64>,
  pub period_min: Option<f64>,
  pub lifespan_years: Option<f64>,
  pub epoch: Option<String>,
  pub mean_motion: Option<f64>,
  pub raan: Option<f64>,
  pub arg_of_pericenter: Option<f64>,
  pub mean_anomaly: Option<f64>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="payloads"]
pub struct UpdatePayloads<'a> {
  pub payload_id: &'a str,
  pub name: &'a String,
  pub type_: &'a String,
  pub reused: &'a bool,
  pub launch: &'a String,
  pub customers: &'a Vec<String>,
  pub norad_ids: &'a Vec<i32>,
  pub nationalities: &'a Vec<String>,
  pub manufacturers: &'a Vec<String>,
  pub mass_kg: &'a f64,
  pub mass_lbs: &'a f64,
  pub orbit: &'a String,
  pub reference_system: &'a String,
  pub regime: &'a String,
  pub longitude: &'a f64,
  pub semi_major_axis_km: &'a f64,
  pub eccentricity: &'a f64,
  pub periapsis_km: &'a f64,
  pub apoapsis_km: &'a f64,
  pub inclination_deg: &'a f64,
  pub period_min: &'a f64,
  pub lifespan_years: &'a f64,
  pub epoch: &'a String,
  pub mean_motion: &'a f64,
  pub raan: &'a f64,
  pub arg_of_pericenter: &'a f64,
  pub mean_anomaly: &'a f64,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="payload_dragon"]
pub struct UpdatePayloadDragon<'a> {
  pub payload_id: &'a str,
  pub capsule: &'a String,
  pub mass_returned_kg: &'a f64,
  pub mass_returned_lbs: &'a f64,
  pub flight_time_sec: &'a i32,
  pub manifest: &'a String,
  pub water_landing: &'a bool,
  pub land_landing: &'a bool,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketFirstStage {
  pub thrust_sea_level: Thrust,
  pub thrust_vacuum: Thrust,
  pub reusable: Option<bool>,
  pub engines: Option<i32>,
  pub fuel_amount_tons: Option<f64>,
  pub burn_time_sec: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketSecondStage {
  pub thrust: Thrust,
  pub payloads: RocketSecondStagePayloads,
  pub reusable: Option<bool>,
  pub engines: Option<i32>,
  pub fuel_amount_tons: Option<f64>,
  pub burn_time_sec: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketSecondStagePayloadsFairing {
  pub height: Distance,
  pub diameter: Distance,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketSecondStagePayloads {
  pub composite_fairing: RocketSecondStagePayloadsFairing,
  pub option_1: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketEngineIsp {
  pub sea_level: Option<i32>,
  pub vacuum: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketEngine {
  pub isp: RocketEngineIsp,
  pub thrust_sea_level: Thrust,
  pub thrust_vacuum: Thrust,
  pub number: Option<i32>,
  pub r#type: Option<String>,
  pub version: Option<String>,
  pub layout: Option<String>,
  pub engine_loss_max: Option<i32>,
  pub propellant_1: Option<String>,
  pub propellant_2: Option<String>,
  pub thrust_to_weight: Option<f64>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketLandingLegs {
  pub number: Option<i32>,
  pub material: Option<String>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct RocketPayloadWeights {
  pub id: Option<String>,
  pub name: Option<String>,
  pub kg: Option<i32>,
  pub lb: Option<i32>,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Rockets {
  pub id: String,
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub active: Option<bool>,
  pub stages: Option<i32>,
  pub boosters: Option<i32>,
  pub cost_per_launch: Option<i32>,
  pub success_rate_pct: Option<i32>,
  pub first_flight: Option<String>,
  pub country: Option<String>,
  pub company: Option<String>,
  pub wikipedia: Option<String>,
  pub description: Option<String>,
  pub height: Distance,
  pub diameter: Distance,
  pub mass: Mass,
  pub first_stage: RocketFirstStage,
  pub second_stage: RocketSecondStage,
  pub engines: RocketEngine,
  pub landing_legs: RocketLandingLegs,
  pub payload_weights: Vec<RocketPayloadWeights>,
  pub flickr_images: Option<Vec<String>>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="rocket_payload_weights"]
pub struct UpdateRocketPayloadWeight<'a> {
  pub id: &'a str,
  pub rocket_id: &'a str,
  pub payload_id: &'a String,
  pub name: &'a String,
  pub kg: &'a i32,
  pub lb: &'a i32,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="rocket_engine"]
pub struct UpdateRocketEngine<'a> {
  pub rocket_id: &'a str,
  pub isp_sea_level: &'a i32,
  pub isp_vacuum: &'a i32,
  pub thrust_sea_level_kn: &'a f64,
  pub thrust_sea_level_lbf: &'a f64,
  pub thrust_vacuum_kn: &'a f64,
  pub thrust_vacuum_lbf: &'a f64,
  pub number: &'a i32,
  pub type_: &'a String,
  pub version: &'a String,
  pub layout: &'a String,
  pub engine_loss_max: &'a i32,
  pub propellant_1: &'a String,
  pub propellant_2: &'a String,
  pub thrust_to_weight: &'a f64,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="rocket_second_stage"]
pub struct UpdateRocketSecondStage<'a> {
  pub rocket_id: &'a str,
  pub thrust_kn: &'a f64,
  pub thrust_lbf: &'a f64,
  pub payloads_composite_fairing_height_meters: &'a f64,
  pub payloads_composite_fairing_height_feet: &'a f64,
  pub payloads_composite_fairing_diameter_meters: &'a f64,
  pub payloads_composite_fairing_diameter_feet: &'a f64,
  pub payloads_option_1: &'a String,
  pub reusable: &'a bool,
  pub engines: &'a i32,
  pub fuel_amount_tons: &'a f64,
  pub burn_time_sec: &'a i32,
  pub row_updated: &'a DateTime<Utc>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="rocket_first_stage"]
pub struct UpdateRocketFirstStage<'a> {
  pub rocket_id: &'a str,
  pub thrust_sea_level_kn: &'a f64, 
  pub thrust_sea_level_lbf: &'a f64, 
  pub thrust_vacuum_kn: &'a f64, 
  pub thrust_vacuum_lbf: &'a f64, 
  pub reusable: &'a bool, 
  pub engines: &'a i32, 
  pub fuel_amount_tons: &'a f64, 
  pub burn_time_sec: &'a i32, 
  pub row_updated: &'a DateTime<Utc>, 
}
#[derive(Insertable, AsChangeset)]
#[table_name="rockets"]
pub struct UpdateRockets<'a> {
  pub rocket_id: &'a str,
  pub name: &'a String,
  pub type_: &'a String,
  pub active: &'a bool,
  pub stages: &'a i32,
  pub boosters: &'a i32,
  pub cost_per_launch: &'a i32,
  pub success_rate_pct: &'a i32,
  pub first_flight: &'a String,
  pub country: &'a String,
  pub company: &'a String,
  pub wikipedia: &'a String,
  pub description: &'a String,
  pub height_meters: &'a f64,
  pub height_feet: &'a f64,
  pub diameter_meters: &'a f64,
  pub diameter_feet: &'a f64,
  pub mass_kg: &'a i32,
  pub mass_lbs: &'a i32,
  pub landing_legs_number: &'a i32,
  pub landing_legs_material: &'a String,
  pub flickr_images: &'a Vec<String>,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
pub struct Ships {
  pub id: String,
  pub legacy_id: Option<String>,
  pub name: Option<String>,
  pub r#type: Option<String>,
  pub active: Option<bool>,
  pub model: Option<String>,
  pub roles: Option<Vec<String>>,
  pub imo: Option<i32>,
  pub mmsi: Option<i32>,
  pub abs: Option<i32>,
  pub class: Option<i32>,
  pub mass_kg: Option<i32>,
  pub mass_lbs: Option<i32>,
  pub year_built: Option<i32>,
  pub home_port: Option<String>,
  pub status: Option<String>,
  pub speed_kn: Option<i32>,
  pub course_deg: Option<i32>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub last_ais_update: Option<i32>,
  pub link: Option<String>,
  pub image: Option<String>,
  pub launches: Option<Vec<String>>,
}
#[derive(Insertable, AsChangeset)]
#[table_name="ships"]
pub struct UpdateShips<'a> {
  pub id: &'a String,
  pub legacy_id: &'a String,
  pub name: &'a String,
  pub type_: &'a String,
  pub active: &'a bool,
  pub model: &'a String,
  pub roles: &'a Vec<String>,
  pub imo: &'a i32,
  pub mmsi: &'a i32,
  pub abs: &'a i32,
  pub class: &'a i32,
  pub mass_kg: &'a i32,
  pub mass_lbs: &'a i32,
  pub year_built: &'a i32,
  pub home_port: &'a String,
  pub status: &'a String,
  pub speed_kn: &'a i32,
  pub course_deg: &'a i32,
  pub latitude: &'a f64,
  pub longitude: &'a f64,
  pub last_ais_update: &'a i32,
  pub link: &'a String,
  pub image: &'a String,
  pub launches: &'a Vec<String>,
  pub row_updated: &'a DateTime<Utc>,
}


#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
#[allow(non_snake_case)]
pub struct Starlinks {
  pub id: String,
  pub version: Option<String>,
  pub launch: Option<String>,
  pub longitude: Option<f64>,
  pub latitude: Option<f64>,
  pub height_km: Option<f64>,
  pub velocity_kms: Option<f64>,
  pub spaceTrack: SpaceTrack,
}
#[derive(Debug, Deserialize, Queryable, GraphQLObject)]
#[allow(non_snake_case)]
pub struct SpaceTrack {
  pub CCSDS_OMM_VERS: Option<String>,
  pub COMMENT: Option<String>,
  pub CREATION_DATE: Option<String>,
  pub ORIGINATOR: Option<String>,
  pub OBJECT_NAME: Option<String>,
  pub OBJECT_ID: Option<String>,
  pub CENTER_NAME: Option<String>,
  pub REF_FRAME: Option<String>,
  pub TIME_SYSTEM: Option<String>,
  pub MEAN_ELEMENT_THEORY: Option<String>,
  pub EPOCH: Option<String>,
  pub MEAN_MOTION: Option<f64>,
  pub ECCENTRICITY: Option<f64>,
  pub INCLINATION: Option<f64>,
  pub RA_OF_ASC_NODE: Option<f64>,
  pub ARG_OF_PERICENTER: Option<f64>,
  pub MEAN_ANOMALY: Option<f64>,
  pub EPHEMERIS_TYPE: Option<f64>,
  pub CLASSIFICATION_TYPE: Option<String>,
  pub NORAD_CAT_ID: Option<i32>,
  pub ELEMENT_SET_NO: Option<i32>,
  pub REV_AT_EPOCH: Option<i32>,
  pub BSTAR: Option<f64>,
  pub MEAN_MOTION_DOT: Option<f64>,
  pub MEAN_MOTION_DDOT: Option<f64>,
  pub SEMIMAJOR_AXIS: Option<f64>,
  pub PERIOD: Option<f64>,
  pub APOAPSIS: Option<f64>,
  pub PERIAPSIS: Option<f64>,
  pub OBJECT_TYPE: Option<String>,
  pub RCS_SIZE: Option<String>,
  pub COUNTRY_CODE: Option<String>,
  pub LAUNCH_DATE: Option<String>,
  pub SITE: Option<String>,
  pub DECAY_DATE: Option<String>,
  pub DECAYED: Option<i32>,
  pub FILE: Option<i32>,
  pub GP_ID: Option<i32>,
  pub TLE_LINE0: Option<String>,
  pub TLE_LINE1: Option<String>,
  pub TLE_LINE2: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="starlinks"]
pub struct UpdateStarlinks<'a> {
  pub starlink_id: &'a str,
  pub version: &'a String,
  pub launch: &'a String,
  pub longitude: &'a f64,
  pub latitude: &'a f64,
  pub height_km: &'a f64,
  pub velocity_kms: &'a f64,
  pub row_updated: &'a DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name="starlink_spacetrack"]
pub struct UpdateStarlinkSpacetrack<'a> {
  pub starlink_id: &'a str,
  pub ccsds_omm_vers: &'a String,
  pub comment: &'a String,
  pub creation_date: &'a String,
  pub originator: &'a String,
  pub object_name: &'a String,
  pub object_id: &'a String,
  pub center_name: &'a String,
  pub ref_frame: &'a String,
  pub time_system: &'a String,
  pub mean_element_theory: &'a String,
  pub epoch: &'a String,
  pub mean_motion: &'a f64,
  pub eccentricity: &'a f64,
  pub inclination: &'a f64,
  pub ra_of_asc_node: &'a f64,
  pub arg_of_pericenter: &'a f64,
  pub mean_anomaly: &'a f64,
  pub ephemeris_type: &'a f64,
  pub classification_type: &'a String,
  pub norad_cat_id: &'a i32,
  pub element_set_no: &'a i32,
  pub rev_at_epoch: &'a i32,
  pub bstar: &'a f64,
  pub mean_motion_dot: &'a f64,
  pub mean_motion_ddot: &'a f64,
  pub semimajor_axis: &'a f64,
  pub period: &'a f64,
  pub apoapsis: &'a f64,
  pub periapsis: &'a f64,
  pub object_type: &'a String,
  pub rcs_size: &'a String,
  pub country_code: &'a String,
  pub launch_date: &'a String,
  pub site: &'a String,
  pub decay_date: &'a String,
  pub decayed: &'a i32,
  pub file: &'a i32,
  pub gp_id: &'a i32,
  pub tle_line0: &'a String,
  pub tle_line1: &'a String,
  pub tle_line2: &'a String,
  pub row_updated: &'a DateTime<Utc>,
}