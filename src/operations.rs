use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{Date, DateTime, Utc};


use crate::database::models::*;
use crate::database::schema::*;

// Database connection PgConnection
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
) {
  let now: DateTime<Utc> = Utc::now();

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
    flickr_images: &roadster_details.flickr_images.as_ref().unwrap(),
    row_updated: &now,
  };

  diesel::insert_into(roadster_info::table)
    .values(&new_roadster_entry)
    .on_conflict(roadster_info::id)
    .do_update()
    .set(&new_roadster_entry)
    .execute(conn)
    .expect("Error saving entry to Roadster_info!");
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
  let now: DateTime<Utc> = Utc::now();

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
    row_updated: &now,
  };

  diesel::insert_into(company_info::table)
    .values(&new_company_entry)
    .on_conflict(company_info::id)
    .do_update()
    .set(&new_company_entry)
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

// Method to push a row into database table['capsules']
pub fn add_capsule<'a>(
  conn: &PgConnection,
  capsule_details: &'a Capsules
) {

  let now: DateTime<Utc> = Utc::now();

  let new_capsule = UpdateCapsules {
    id: &capsule_details.id,
    reuse_count: &capsule_details.reuse_count.unwrap(),
    water_landings: &capsule_details.water_landings.unwrap(),
    land_landings: &capsule_details.land_landings.unwrap(),
    last_update: &capsule_details.last_update.clone().unwrap_or(String::from("No update found")),
    launches: &capsule_details.launches.as_ref().unwrap(),
    serial: &capsule_details.serial.as_ref().unwrap(),
    status: &capsule_details.status.as_ref().unwrap(),
    type_: &capsule_details.r#type.as_ref().unwrap(),
    row_updated: &now,
  };

  // Push row into capsules table, if a conflict is found the row is updated.
  diesel::insert_into(capsules::table)
    .values(&new_capsule)
    .on_conflict(capsules::id)
    .do_update()
    // .on_conflict_do_nothing()
    .set(&new_capsule)
    .execute(conn)
    .expect("Error saving entry to capsules table");
}

// Method to push row into database table['cores']
pub fn add_core<'a>(
  conn: &PgConnection,
  core_details: &'a Cores
) {
  let now: DateTime<Utc> = Utc::now();

  let new_core = UpdateCores {
    id: &core_details.id,
    block: &core_details.block.clone().unwrap_or(0),
    reuse_count: &core_details.reuse_count.unwrap(),
    rtls_attempts: &core_details.rtls_attempts.unwrap(),
    rtls_landings: &core_details.rtls_landings.unwrap(),
    asds_attempts: &core_details.asds_attempts.unwrap(),
    asds_landings: &core_details.asds_landings.unwrap(),
    last_update: &core_details.last_update.clone().unwrap_or(String::from("No update found")),
    launches: &core_details.launches.as_ref().unwrap(),
    serial: &core_details.serial.as_ref().unwrap(),
    status: &core_details.status.as_ref().unwrap(),
    row_updated: &now,
  };

  // Push new row into cores table, if a conflict is found the row is updated.
  diesel::insert_into(cores::table)
    .values(&new_core)
    .on_conflict(cores::id)
    .do_update()
    .set(&new_core)
    .execute(conn)
    .expect("Error saving entry into cores table");
}

// Method to push row into database table['crew']
pub fn add_crew_member<'a>(
  conn: &PgConnection,
  crew_details: &'a Crew
) {
  let now: DateTime<Utc> = Utc::now();

  let new_member = UpdateCrew {
    id: &crew_details.id,
    name: &crew_details.name.as_ref().unwrap(),
    agency: &crew_details.agency.as_ref().unwrap(),
    image: &crew_details.image.as_ref().unwrap(),
    wikipedia: &crew_details.wikipedia.as_ref().unwrap(),
    launches: &crew_details.launches.as_ref().unwrap(),
    status: &crew_details.status.as_ref().unwrap(),
    row_updated: &now,
  };

  diesel::insert_into(crew::table)
    .values(&new_member)
    .on_conflict(crew::id)
    .do_update()
    .set(&new_member)
    .execute(conn)
    .expect("Error saving entry into crew table");
}

// Method to push row into database tables['dragons', 'dragons_heat_shield', 'dragons_pressurized_capsule', 'dragons_trunk', 'dragons_thrusters']
pub fn add_dragon<'a>(
  conn: &PgConnection,
  dragon_details: &'a Dragons
) {
  let now: DateTime<Utc> = Utc::now();

  let new_dragon_entry = UpdateDragons {
    dragon_id: &dragon_details.id,
    launch_payload_mass_kg: &dragon_details.launch_payload_mass.kg.unwrap(),
    launch_payload_mass_lbs: &dragon_details.launch_payload_mass.lb.unwrap(),
    launch_payload_vol_cubic_meters: &dragon_details.launch_payload_vol.cubic_meters.unwrap(),
    launch_payload_vol_cubic_feet: &dragon_details.launch_payload_vol.cubic_feet.unwrap(),
    return_payload_mass_kg: &dragon_details.return_payload_mass.kg.unwrap(),
    return_payload_mass_lbs: &dragon_details.return_payload_mass.lb.unwrap(),
    return_payload_vol_cubic_meters: &dragon_details.return_payload_vol.cubic_meters.unwrap(),
    return_payload_vol_cubic_feet: &dragon_details.return_payload_vol.cubic_feet.unwrap(),
    height_w_trunk_meters: &dragon_details.height_w_trunk.meters.unwrap(),
    height_w_trunk_feet: &dragon_details.height_w_trunk.feet.unwrap(),
    diameter_meters: &dragon_details.diameter.meters.unwrap(),
    diameter_feet: &dragon_details.diameter.feet.unwrap(),
    first_flight: &dragon_details.first_flight.as_ref().unwrap(),
    flickr_images: &dragon_details.flickr_images.as_ref().unwrap(),
    name: &dragon_details.name.as_ref().unwrap(),
    type_: &dragon_details.r#type.as_ref().unwrap(),
    active: &dragon_details.active,
    crew_capacity: &dragon_details.crew_capacity.unwrap(),
    sidewall_angle_deg: &dragon_details.sidewall_angle_deg.unwrap(),
    orbit_duration_yr: &dragon_details.orbit_duration_yr.unwrap(),
    dry_mass_kg: &dragon_details.dry_mass_kg.unwrap(),
    dry_mass_lbs: &dragon_details.dry_mass_lbs.unwrap(),
    wikipedia: &dragon_details.wikipedia.as_ref().unwrap(),
    description: &dragon_details.description.as_ref().unwrap(),
    row_updated: &now,
  };

  
}