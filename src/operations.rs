use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
// use std::convert::TryFrom;

use crate::database::models::*;
use crate::database::schema::*;

// struct BigIntegerValue(i64);
// impl TryFrom<i64> for BigIntegerValue {
//   type Error = &'static str;
//   fn try_from(value: i64) -> Result<Self, Self::Error> {
//     if value >= 1000000 {

//     } else {
//       Ok()
//     }
//   }
// } 

// Database connection PgConnection
pub fn establish_connection() -> PgConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

  PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

// Method to add roadster_info to database table['roadster_info']
pub fn create_roadster<'a>(conn: &PgConnection, roadster_details: &'a Roadster) {
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
pub fn delete_roadster_by_id<'a>(conn: &PgConnection, roadster_details: &'a Roadster) {
  use roadster_info::dsl::roadster_info;

  let roadster_id = &roadster_details.id;

  let roadster_to_delete = diesel::delete(roadster_info.find(roadster_id))
    .execute(conn)
    .expect("Unable to delete Roadster entry");

  println!("Deleted {} roadster entry", roadster_to_delete);
}

// Method to add company_info to database table['company_info']
pub fn create_company<'a>(conn: &PgConnection, company_details: &'a Company) {
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
    valuation: &(company_details.valuation.unwrap() as f64),
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
pub fn delete_company_by_id<'a>(conn: &PgConnection, company_details: &'a Company) {
  use company_info::dsl::company_info;
  let company_row_id = &company_details.id;

  let company_to_delete = diesel::delete(company_info.find(company_row_id))
    .execute(conn)
    .expect("Unable to delete Company entry");
  println!("Deleted {} Company entry", company_to_delete);
}

// Method to push a row into database table['capsules']
pub fn add_capsule<'a>(conn: &PgConnection, capsule_details: &'a Capsules) {
  let now: DateTime<Utc> = Utc::now();

  let new_capsule = UpdateCapsules {
    id: &capsule_details.id,
    reuse_count: &capsule_details.reuse_count.unwrap(),
    water_landings: &capsule_details.water_landings.unwrap(),
    land_landings: &capsule_details.land_landings.unwrap(),
    last_update: &capsule_details
      .last_update
      .clone()
      .unwrap_or(String::from("No update found")),
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
pub fn add_core<'a>(conn: &PgConnection, core_details: &'a Cores) {
  let now: DateTime<Utc> = Utc::now();

  let new_core = UpdateCores {
    id: &core_details.id,
    block: &core_details.block.clone().unwrap_or(0),
    reuse_count: &core_details.reuse_count.unwrap(),
    rtls_attempts: &core_details.rtls_attempts.unwrap(),
    rtls_landings: &core_details.rtls_landings.unwrap(),
    asds_attempts: &core_details.asds_attempts.unwrap(),
    asds_landings: &core_details.asds_landings.unwrap(),
    last_update: &core_details
      .last_update
      .clone()
      .unwrap_or(String::from("No update found")),
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
pub fn add_crew_member<'a>(conn: &PgConnection, crew_details: &'a Crew) {
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
pub fn add_dragon<'a>(conn: &PgConnection, dragon_details: &'a Dragons) {
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
    dry_mass_lbs: &dragon_details.dry_mass_lb.unwrap(),
    wikipedia: &dragon_details.wikipedia.as_ref().unwrap(),
    description: &dragon_details.description.as_ref().unwrap(),
    row_updated: &now,
  };

  let new_dragon_heat_shield_entry = UpdateDragonsHeatShield {
    dragon_id: &dragon_details.id,
    material: &dragon_details.heat_shield.material.as_ref().unwrap(),
    size_meters: &dragon_details.heat_shield.size_meters.unwrap(),
    temp_degrees: &dragon_details.heat_shield.temp_degrees.unwrap(),
    dev_partner: &dragon_details.heat_shield.dev_partner.as_ref().unwrap(),
    row_updated: &now,
  };

  let new_dragon_pressurized_capsule_entry = UpdateDragonsPressurizedCapsule {
    dragon_id: &dragon_details.id,
    payload_volume_cubic_meters: &dragon_details
      .pressurized_capsule
      .payload_volume
      .cubic_meters
      .unwrap(),
    payload_volume_cubic_feet: &dragon_details
      .pressurized_capsule
      .payload_volume
      .cubic_feet
      .unwrap(),
    row_updated: &now,
  };

  let new_dragon_trunk_entry = UpdateDragonsTrunk {
    dragon_id: &dragon_details.id,
    trunk_volume_cubic_meters: &dragon_details.trunk.trunk_volume.cubic_meters.unwrap(),
    trunk_volume_cubic_feet: &dragon_details.trunk.trunk_volume.cubic_feet.unwrap(),
    cargo_solar_array: &dragon_details.trunk.cargo.solar_array.unwrap(),
    cargo_unpressurized_cargo: &dragon_details.trunk.cargo.unpressurized_cargo,
    row_updated: &now,
  };

  diesel::insert_into(dragons::table)
    .values(&new_dragon_entry)
    .on_conflict(dragons::dragon_id)
    .do_update()
    .set(&new_dragon_entry)
    .execute(conn)
    .expect("Error saving entry into dragons table");

  diesel::insert_into(dragons_heat_shield::table)
    .values(&new_dragon_heat_shield_entry)
    .on_conflict(dragons_heat_shield::dragon_id)
    .do_update()
    .set(&new_dragon_heat_shield_entry)
    .execute(conn)
    .expect("Error saving entry into dragons_heat_shield table");

  diesel::insert_into(dragons_pressurized_capsule::table)
    .values(&new_dragon_pressurized_capsule_entry)
    .on_conflict(dragons_pressurized_capsule::dragon_id)
    .do_update()
    .set(&new_dragon_pressurized_capsule_entry)
    .execute(conn)
    .expect("Error saving entry into dragons_pressurized_capsule table");

  diesel::insert_into(dragons_trunk::table)
    .values(&new_dragon_trunk_entry)
    .on_conflict(dragons_trunk::dragon_id)
    .do_update()
    .set(&new_dragon_trunk_entry)
    .execute(conn)
    .expect("Error saving entry into dragons_trunk table");

  for (index, thruster) in dragon_details.thrusters.iter().enumerate() {
    let new_dragon_thrusters_entry = UpdateDragonsThrusters {
      id: &format!("{}{}{}", &dragon_details.id, "_", index.to_string()),
      dragon_id: &dragon_details.id,
      type_: &thruster.r#type.as_ref().unwrap(),
      amount: &thruster.amount.unwrap(),
      pods: &thruster.pods.unwrap(),
      fuel_1: &thruster.fuel_1.as_ref().unwrap(),
      fuel_2: &thruster.fuel_2.as_ref().unwrap(),
      isp: &thruster.isp.unwrap(),
      thrust_kn: &thruster.thrust.kN.unwrap(),
      thrust_lbf: &thruster.thrust.lbf.unwrap(),
      row_updated: &now,
    };

    diesel::insert_into(dragons_thrusters::table)
      .values(&new_dragon_thrusters_entry)
      .on_conflict(dragons_thrusters::id)
      .do_update()
      .set(&new_dragon_thrusters_entry)
      .execute(conn)
      .expect("Error saving entry into dragons_thrusters table");
  }
}

// Method to push row into database table['history']
pub fn add_history<'a>(conn: &PgConnection, history_details: &'a History) {
  let now: DateTime<Utc> = Utc::now();

  let new_history_entry = UpdateHistory {
    id: &history_details.id,
    title: &history_details.title.as_ref().unwrap(),
    event_date_utc: &history_details.event_date_utc.as_ref().unwrap(),
    event_date_unix: &history_details.event_date_unix.unwrap(),
    details: &history_details.details.as_ref().unwrap(),
    links_article: &history_details
      .links
      .article
      .clone()
      .unwrap_or("".to_string()),
    row_updated: &now,
  };

  diesel::insert_into(history::table)
    .values(&new_history_entry)
    .on_conflict(history::id)
    .do_update()
    .set(&new_history_entry)
    .execute(conn)
    .expect("Error saving entry into history table");
}

// Method to push row into database table['landpads']
pub fn add_landpad<'a>(conn: &PgConnection, landpad_details: &'a Landpads) {
  let now: DateTime<Utc> = Utc::now();

  let new_landpad_entry = UpdateLandpads {
    id: &landpad_details.id,
    name: &landpad_details.name.as_ref().unwrap(),
    full_name: &landpad_details.full_name.as_ref().unwrap(),
    status: &landpad_details.status.as_ref().unwrap(),
    type_: &landpad_details.r#type.as_ref().unwrap(),
    locality: &landpad_details.locality.as_ref().unwrap(),
    region: &landpad_details.region.as_ref().unwrap(),
    latitude: &landpad_details.latitude.unwrap(),
    longitude: &landpad_details.longitude.unwrap(),
    landing_attempts: &landpad_details.landing_attempts.unwrap(),
    landing_successes: &landpad_details.landing_successes.unwrap(),
    wikipedia: &landpad_details.wikipedia.as_ref().unwrap(),
    details: &landpad_details.details.as_ref().unwrap(),
    launches: &landpad_details.launches.as_ref().unwrap(),
    row_updated: &now,
  };

  diesel::insert_into(landpads::table)
    .values(&new_landpad_entry)
    .on_conflict(landpads::id)
    .do_update()
    .set(&new_landpad_entry)
    .execute(conn)
    .expect("Error saving entry into landpads table");
}


// Method to push row into database tables['launches', 'launch_fairings', 'launch_links', 'launch_cores', 'launch_links']
pub fn add_launch<'a>(conn: &PgConnection, launch_details: &'a Launches) {
  let now: DateTime<Utc> = Utc::now();

  let new_launch_entry = UpdateLaunches {
    launch_id: &launch_details.id,
    name: &launch_details.name.as_ref().unwrap(),
    static_fire_date_utc: &launch_details.static_fire_date_utc.clone().unwrap_or("".to_string()),
    static_fire_date_unix: &launch_details.static_fire_date_unix.unwrap_or(0),
    tdb: &launch_details.tdb.unwrap_or(false),
    net: &launch_details.net.unwrap_or(false),
    window_number: &launch_details.window.unwrap_or(0),
    rocket: &launch_details.rocket.as_ref().unwrap(),
    success: &launch_details.success.unwrap_or(false),
    details: &launch_details.details.clone().unwrap_or("".to_string()),
    crew: &launch_details.crew.as_ref().unwrap(),
    ships: &launch_details.ships.as_ref().unwrap(),
    capsules: &launch_details.capsules.as_ref().unwrap(),
    payloads: &launch_details.payloads.as_ref().unwrap(),
    launchpad: &launch_details.launchpad.as_ref().unwrap(),
    auto_update: &launch_details.auto_update.unwrap_or(false),
    flight_number: &launch_details.flight_number.unwrap(),
    date_utc: &launch_details.date_utc.as_ref().unwrap(),
    date_unix: &launch_details.date_unix.unwrap(),
    date_local: &launch_details.date_local.as_ref().unwrap(),
    date_precision: &launch_details.date_precision.as_ref().unwrap(),
    upcoming: &launch_details.upcoming.unwrap_or(false),
    row_updated: &now,
  };
  let placeholder_fairing = LaunchFairings {
    reused: None,
    recovery_attempt: None,
    recovered: None,
    ships: None,
  };
  let new_launch_fairing_entry = UpdateLaunchFairings {
    launch_id: &launch_details.id,
    reused: &launch_details.fairings.as_ref().unwrap_or(&placeholder_fairing).reused.unwrap_or(false),
    recovery_attempt: &launch_details.fairings.as_ref().unwrap_or(&placeholder_fairing).recovery_attempt.unwrap_or(false),
    recovered: &launch_details.fairings.as_ref().unwrap_or(&placeholder_fairing).recovered.unwrap_or(false),
    ships: &launch_details.fairings.as_ref().unwrap_or(&placeholder_fairing).ships.clone().unwrap_or([].to_vec()),
    row_updated: &now,
  };
  let placeholder_link = LaunchLinks {
    patch: None,
    reddit: None,
    flickr: None,
    presskit: None,
    webcast: None,
    youtube_id: None,
    article: None,
    wikipedia: None,
  };
  let placeholder_patch = LaunchLinkPatch {
    small: None,
    large: None,
  };
  let new_launch_links_entry = UpdateLaunchLinks {
    launch_id: &launch_details.id,
    patch_small: &launch_details.links.as_ref().unwrap_or(&placeholder_link).patch.as_ref().unwrap_or(&placeholder_patch).small.clone().unwrap_or("".to_string()),
    patch_large: &launch_details.links.as_ref().unwrap_or(&placeholder_link).patch.as_ref().unwrap_or(&placeholder_patch).large.clone().unwrap_or("".to_string()),
    reddit_campaign: &launch_details.links.as_ref().unwrap_or(&placeholder_link).reddit.as_ref().unwrap().campaign.clone().unwrap_or("".to_string()),
    reddit_launch: &launch_details.links.as_ref().unwrap_or(&placeholder_link).reddit.as_ref().unwrap().launch.clone().unwrap_or("".to_string()),
    reddit_media: &launch_details.links.as_ref().unwrap_or(&placeholder_link).reddit.as_ref().unwrap().media.clone().unwrap_or("".to_string()),
    reddit_recovery: &launch_details.links.as_ref().unwrap_or(&placeholder_link).reddit.as_ref().unwrap().recovery.clone().unwrap_or("".to_string()),
    flickr_small: &launch_details.links.as_ref().unwrap_or(&placeholder_link).flickr.as_ref().unwrap().small.as_ref().unwrap(),
    flickr_original: &launch_details.links.as_ref().unwrap_or(&placeholder_link).flickr.as_ref().unwrap().original.as_ref().unwrap(),
    presskit: &launch_details.links.as_ref().unwrap_or(&placeholder_link).presskit.clone().unwrap_or("".to_string()),
    webcast: &launch_details.links.as_ref().unwrap_or(&placeholder_link).webcast.clone().unwrap_or("".to_string()),
    youtube_id: &launch_details.links.as_ref().unwrap_or(&placeholder_link).youtube_id.clone().unwrap_or("".to_string()),
    article: &launch_details.links.as_ref().unwrap_or(&placeholder_link).article.clone().unwrap_or("".to_string()),
    wikipedia: &launch_details.links.as_ref().unwrap_or(&placeholder_link).wikipedia.clone().unwrap_or("".to_string()),
    row_updated: &now,
  };

  diesel::insert_into(launches::table)
    .values(&new_launch_entry)
    .on_conflict(launches::launch_id)
    .do_update()
    .set(&new_launch_entry)
    .execute(conn)
    .expect("Error saving entry into launches table");

  diesel::insert_into(launch_fairings::table)
    .values(&new_launch_fairing_entry)
    .on_conflict(launch_fairings::launch_id)
    .do_update()
    .set(&new_launch_fairing_entry)
    .execute(conn)
    .expect("Error saving entry into launch_fairings");

  diesel::insert_into(launch_links::table)
    .values(&new_launch_links_entry)  
    .on_conflict(launch_links::launch_id)
    .do_update()
    .set(&new_launch_links_entry)
    .execute(conn)
    .expect("Error saving entry into launch_links table");

  for (index, failure) in launch_details.failures.as_ref().unwrap().iter().enumerate() {
    let new_launch_failures_entry = UpdateLaunchFailures {
      id: &format!("{}{}{}", &launch_details.id, "_", index.to_string()),
      launch_id: &launch_details.id,
      time: &failure.time.unwrap(),
      altitude: &failure.altitude.unwrap_or(0),
      reason: &failure.reason.as_ref().unwrap(),
      row_updated: &now,
    };
    diesel::insert_into(launch_failures::table)
      .values(&new_launch_failures_entry)
      .on_conflict(launch_failures::id)
      .do_update()
      .set(&new_launch_failures_entry)
      .execute(conn)
      .expect("Error saving entry into launch_failures table");
  };

  for (index, core) in launch_details.cores.as_ref().unwrap().iter().enumerate() {
    let new_launch_core_entry = UpdateLaunchCores {
      id: &format!("{}{}{}", &launch_details.id, "_", index.to_string()),
      launch_id: &launch_details.id,
      core: &core.core.clone().unwrap_or("".to_string()),
      flight: &core.flight.unwrap_or(0),
      gridfins: &core.gridfins.unwrap_or(false),
      legs: &core.legs.unwrap_or(false),
      reused: &core.reused.unwrap_or(false),
      landing_attempt: &core.landing_attempt.unwrap_or(false),
      landing_success: &core.landing_success.unwrap_or(false),
      landing_type: &core.landing_type.clone().unwrap_or("".to_string()),
      landpad: &core.landpad.clone().unwrap_or("".to_string()),
      row_updated: &now,
    };
    diesel::insert_into(launch_cores::table)
      .values(&new_launch_core_entry)
      .on_conflict(launch_cores::id)
      .do_update()
      .set(&new_launch_core_entry)
      .execute(conn)
      .expect("Error saving entry into launch_cores table");
  };
}


// Method to push row into database table['launchpads']
pub fn add_launchpad<'a>(conn: &PgConnection, launchpad_details: &'a LaunchPads) {
  let now: DateTime<Utc> = Utc::now();

  let new_launchpad_entry = UpdateLaunchpads {
    id: &launchpad_details.id,
    name: &launchpad_details.name.as_ref().unwrap(),
    full_name: &launchpad_details.full_name.as_ref().unwrap(),
    locality: &launchpad_details.locality.as_ref().unwrap(),
    region: &launchpad_details.region.as_ref().unwrap(),
    timezone: &launchpad_details.timezone.as_ref().unwrap(),
    latitude: &launchpad_details.latitude.unwrap(),
    longitude: &launchpad_details.longitude.unwrap(),
    launch_attempts: &launchpad_details.launch_attempts.unwrap(),
    launch_successes: &launchpad_details.launch_successes.unwrap(),
    rockets: &launchpad_details.rockets.as_ref().unwrap(),
    launches: &launchpad_details.launches.as_ref().unwrap(),
    status: &launchpad_details.status.as_ref().unwrap(),
    row_updated: &now,
  };

  diesel::insert_into(launchpads::table)
    .values(&new_launchpad_entry)
    .on_conflict(launchpads::id)
    .do_update()
    .set(&new_launchpad_entry)
    .execute(conn)
    .expect("Error saving entry into launchpads table");
}


// Method to push row into database table['payloads']
pub fn add_payload<'a>(conn: &PgConnection, payload_details: &'a Payloads) {
  let now: DateTime<Utc> = Utc::now();

  let new_payload_entry = UpdatePayloads {
    payload_id: &payload_details.id,
    name: &payload_details.name.as_ref().unwrap(),
    type_: &payload_details.r#type.as_ref().unwrap(),
    reused: &payload_details.reused.as_ref().unwrap(),
    launch: &payload_details.launch.as_ref().unwrap(),
    customers: &payload_details.customers.as_ref().unwrap(),
    norad_ids: &payload_details.norad_ids.as_ref().unwrap(),
    nationalities: &payload_details.nationalities.as_ref().unwrap(),
    manufacturers: &payload_details.manufacturers.as_ref().unwrap(),
    mass_kg: &payload_details.mass_kg.unwrap_or(0.0),
    mass_lbs: &payload_details.mass_lbs.unwrap_or(0.0),
    orbit: &payload_details.orbit.clone().unwrap_or("".to_string()),
    reference_system: &payload_details.reference_system.clone().unwrap_or("".to_string()),
    regime: &payload_details.regime.clone().unwrap_or("".to_string()),
    longitude: &payload_details.longitude.unwrap_or(0.0),
    semi_major_axis_km: &payload_details.semi_major_axis_km.unwrap_or(0.0),
    eccentricity: &payload_details.eccentricity.unwrap_or(0.0),
    periapsis_km: &payload_details.periapsis_km.unwrap_or(0.0),
    apoapsis_km: &payload_details.apoapsis_km.unwrap_or(0.0),
    inclination_deg: &payload_details.inclination_deg.unwrap_or(0.0),
    period_min: &payload_details.period_min.unwrap_or(0.0),
    lifespan_years: &payload_details.lifespan_years.unwrap_or(0.0),
    epoch: &payload_details.epoch.clone().unwrap_or("".to_string()),
    mean_motion: &payload_details.mean_motion.unwrap_or(0.0),
    raan: &payload_details.raan.unwrap_or(0.0),
    arg_of_pericenter: &payload_details.arg_of_pericenter.unwrap_or(0.0),
    mean_anomaly: &payload_details.mean_anomaly.unwrap_or(0.0),
    row_updated: &now,
  };
  let new_payload_dragon_entry = UpdatePayloadDragon {
    payload_id: &payload_details.id,
    capsule: &payload_details.dragon.capsule.clone().unwrap_or("".to_string()),
    mass_returned_kg: &payload_details.dragon.mass_returned_kg.unwrap_or(0.0),
    mass_returned_lbs: &payload_details.dragon.mass_returned_lbs.unwrap_or(0.0),
    flight_time_sec: &payload_details.dragon.flight_time_sec.unwrap_or(0),
    manifest: &payload_details.dragon.manifest.clone().unwrap_or("".to_string()),
    water_landing: &payload_details.dragon.water_landing.clone().unwrap_or(false),
    land_landing: &payload_details.dragon.land_landing.clone().unwrap_or(false),
    row_updated: &now,
  };
  

  diesel::insert_into(payloads::table)
    .values(&new_payload_entry)
    .on_conflict(payloads::payload_id)
    .do_update()
    .set(&new_payload_entry)
    .execute(conn)
    .expect("Error saving entry into payloads table");

  diesel::insert_into(payload_dragon::table)
  .values(&new_payload_dragon_entry)
  .on_conflict(payload_dragon::payload_id)
  .do_update()
  .set(&new_payload_dragon_entry)
  .execute(conn)
  .expect("Error saving entry into payload_dragon table");
}


// Method to push row into database tables['rockets', 'rocket_first_stage', 'rocket_second_stage', 'rocket_engine', 'rocket_payload_weights']
pub fn add_rocket<'a>(conn: &PgConnection, rocket_details: &'a Rockets) {
  let now: DateTime<Utc> = Utc::now();

  let new_rocket_entry = UpdateRockets {
    rocket_id: &rocket_details.id,
    name: &rocket_details.name.clone().unwrap_or("".to_string()),
    type_: &rocket_details.r#type.clone().unwrap_or("".to_string()),
    active: &rocket_details.active.unwrap_or(false),
    stages: &rocket_details.stages.unwrap_or(0),
    boosters: &rocket_details.boosters.unwrap_or(0),
    cost_per_launch: &rocket_details.cost_per_launch.unwrap_or(0),
    success_rate_pct: &rocket_details.success_rate_pct.unwrap_or(0),
    first_flight: &rocket_details.first_flight.clone().unwrap_or("".to_string()),
    country: &rocket_details.country.clone().unwrap_or("".to_string()),
    company: &rocket_details.company.clone().unwrap_or("".to_string()),
    wikipedia: &rocket_details.wikipedia.clone().unwrap_or("".to_string()),
    description: &rocket_details.description.clone().unwrap_or("".to_string()),
    height_meters: &rocket_details.height.meters.unwrap_or(0.0),
    height_feet: &rocket_details.height.feet.unwrap_or(0.0),
    diameter_meters: &rocket_details.diameter.meters.unwrap_or(0.0),
    diameter_feet: &rocket_details.diameter.feet.unwrap_or(0.0),
    mass_kg: &rocket_details.mass.kg.unwrap_or(0),
    mass_lbs: &rocket_details.mass.lb.unwrap_or(0),
    landing_legs_number: &rocket_details.landing_legs.number.unwrap_or(0),
    landing_legs_material: &rocket_details.landing_legs.material.clone().unwrap_or("".to_string()),
    flickr_images: &rocket_details.flickr_images.as_ref().unwrap(),
    row_updated: &now,
  };
  let new_rocket_first_stage_entry = UpdateRocketFirstStage {
    rocket_id: &rocket_details.id,
    thrust_sea_level_kn: &rocket_details.first_stage.thrust_sea_level.kN.unwrap_or(0.0),
    thrust_sea_level_lbf: &rocket_details.first_stage.thrust_sea_level.lbf.unwrap_or(0.0),
    thrust_vacuum_kn: &rocket_details.first_stage.thrust_vacuum.kN.unwrap_or(0.0),
    thrust_vacuum_lbf: &rocket_details.first_stage.thrust_vacuum.lbf.unwrap_or(0.0),
    reusable: &rocket_details.first_stage.reusable.unwrap_or(false),
    engines: &rocket_details.first_stage.engines.unwrap_or(0),
    fuel_amount_tons: &rocket_details.first_stage.fuel_amount_tons.unwrap_or(0.0),
    burn_time_sec: &rocket_details.first_stage.burn_time_sec.unwrap_or(0),
    row_updated: &now,
  };
  let new_rocket_second_stage_entry = UpdateRocketSecondStage {
    rocket_id: &rocket_details.id,
    thrust_kn: &rocket_details.second_stage.thrust.kN.unwrap_or(0.0),
    thrust_lbf: &rocket_details.second_stage.thrust.lbf.unwrap_or(0.0),
    payloads_composite_fairing_height_meters: &rocket_details.second_stage.payloads.composite_fairing.height.meters.unwrap_or(0.0),
    payloads_composite_fairing_height_feet: &rocket_details.second_stage.payloads.composite_fairing.height.feet.unwrap_or(0.0),
    payloads_composite_fairing_diameter_meters: &rocket_details.second_stage.payloads.composite_fairing.diameter.meters.unwrap_or(0.0),
    payloads_composite_fairing_diameter_feet: &rocket_details.second_stage.payloads.composite_fairing.diameter.feet.unwrap_or(0.0),
    payloads_option_1: &rocket_details.second_stage.payloads.option_1.clone().unwrap_or("".to_string()),
    reusable: &rocket_details.second_stage.reusable.unwrap_or(false),
    engines: &rocket_details.second_stage.engines.unwrap_or(0),
    fuel_amount_tons: &rocket_details.second_stage.fuel_amount_tons.unwrap_or(0.0),
    burn_time_sec: &rocket_details.second_stage.burn_time_sec.unwrap_or(0),
    row_updated: &now,
  };
  let new_rocket_engine_entry = UpdateRocketEngine {
    rocket_id: &rocket_details.id,
    isp_sea_level: &rocket_details.engines.isp.sea_level.unwrap_or(0),
    isp_vacuum: &rocket_details.engines.isp.vacuum.unwrap_or(0),
    thrust_sea_level_kn: &rocket_details.engines.thrust_sea_level.kN.unwrap_or(0.0),
    thrust_sea_level_lbf: &rocket_details.engines.thrust_sea_level.lbf.unwrap_or(0.0),
    thrust_vacuum_kn: &rocket_details.engines.thrust_vacuum.kN.unwrap_or(0.0),
    thrust_vacuum_lbf: &rocket_details.engines.thrust_vacuum.lbf.unwrap_or(0.0),
    number: &rocket_details.engines.number.unwrap_or(0),
    type_: &rocket_details.engines.r#type.clone().unwrap_or("".to_string()),
    version: &rocket_details.engines.version.clone().unwrap_or("".to_string()),
    layout: &rocket_details.engines.layout.clone().unwrap_or("".to_string()),
    engine_loss_max: &rocket_details.engines.engine_loss_max.unwrap_or(0),
    propellant_1: &rocket_details.engines.propellant_1.clone().unwrap_or("".to_string()),
    propellant_2: &rocket_details.engines.propellant_2.clone().unwrap_or("".to_string()),
    thrust_to_weight: &rocket_details.engines.thrust_to_weight.unwrap_or(0.0),
    row_updated: &now,
  };

  diesel::insert_into(rockets::table)
  .values(&new_rocket_entry)
  .on_conflict(rockets::rocket_id)
  .do_update()
  .set(&new_rocket_entry)
  .execute(conn)
  .expect("Error saving entry into rockets table");

  diesel::insert_into(rocket_first_stage::table)
  .values(&new_rocket_first_stage_entry)
  .on_conflict(rocket_first_stage::rocket_id)
  .do_update()
  .set(&new_rocket_first_stage_entry)
  .execute(conn)
  .expect("Error saving entry into rocket_first_stage table");

  diesel::insert_into(rocket_second_stage::table)
  .values(&new_rocket_second_stage_entry)
  .on_conflict(rocket_second_stage::rocket_id)
  .do_update()
  .set(&new_rocket_second_stage_entry)
  .execute(conn)
  .expect("Error saving entry into rocket_second_stage table");

  diesel::insert_into(rocket_engine::table)
    .values(&new_rocket_engine_entry)
    .on_conflict(rocket_engine::rocket_id)
    .do_update()
    .set(&new_rocket_engine_entry)
    .execute(conn)
    .expect("Error saving entry into rocket_engine table");

    for (index, pw) in rocket_details.payload_weights.iter().enumerate() {
      let new_rocket_payload_weight_entry = UpdateRocketPayloadWeight {
        id: &format!("{}{}{}", &rocket_details.id, "_", index.to_string()),
        rocket_id: &rocket_details.id,
        payload_id: &pw.id.as_ref().unwrap(),
        name: &pw.name.as_ref().unwrap(),
        kg: &pw.kg.unwrap(),
        lb: &pw.lb.unwrap(),
        row_updated: &now,
      };
      
      diesel::insert_into(rocket_payload_weights::table)
        .values(&new_rocket_payload_weight_entry)
        .on_conflict(rocket_payload_weights::id)
        .do_update()
        .set(&new_rocket_payload_weight_entry)
        .execute(conn)
        .expect("Error saving entry into rocket_payload_weights table");
    };
}

// Method to push row into database table['ships']
pub fn add_ship<'a>(conn: &PgConnection, ship_details: &'a Ships) {
  let now: DateTime<Utc> = Utc::now();

  let new_ship_entry = UpdateShips {
    id: &ship_details.id,
    legacy_id: &ship_details.legacy_id.clone().unwrap_or("".to_string()),
    name: &ship_details.name.clone().unwrap_or("".to_string()),
    type_: &ship_details.r#type.clone().unwrap_or("".to_string()),
    active: &ship_details.active.unwrap_or(false),
    model: &ship_details.model.clone().unwrap_or("".to_string()),
    roles: &ship_details.roles.as_ref().unwrap(),
    imo: &ship_details.imo.unwrap_or(0),
    mmsi: &ship_details.mmsi.unwrap_or(0),
    abs: &ship_details.abs.unwrap_or(0),
    class: &ship_details.class.unwrap_or(0),
    mass_kg: &ship_details.mass_kg.unwrap_or(0),
    mass_lbs: &ship_details.mass_lbs.unwrap_or(0),
    year_built: &ship_details.year_built.unwrap_or(0),
    home_port: &ship_details.home_port.clone().unwrap_or("".to_string()),
    status: &ship_details.status.clone().unwrap_or("".to_string()),
    speed_kn: &ship_details.speed_kn.unwrap_or(0),
    course_deg: &ship_details.course_deg.unwrap_or(0),
    latitude: &ship_details.latitude.unwrap_or(0.0),
    longitude: &ship_details.longitude.unwrap_or(0.0),
    last_ais_update: &ship_details.last_ais_update.unwrap_or(0),
    link: &ship_details.link.clone().unwrap_or("".to_string()),
    image: &ship_details.image.clone().unwrap_or("".to_string()),
    launches: &ship_details.launches.as_ref().unwrap(),
    row_updated: &now,
  };

  diesel::insert_into(ships::table)
    .values(&new_ship_entry)
    .on_conflict(ships::id)
    .do_update()
    .set(&new_ship_entry)
    .execute(conn)
    .expect("Error saving entry into ships table");
}

// Method to push row into database table['starlinks', 'starlink_spacetrack']
pub fn add_starlink<'a>(conn: &PgConnection, starlink_details: &'a Starlinks) {
  let now: DateTime<Utc> = Utc::now();

  let new_starlink_entry = UpdateStarlinks {
    starlink_id: &starlink_details.id,
    version: &starlink_details.version.clone().unwrap_or("".to_string()),
    launch: &starlink_details.launch.clone().unwrap_or("".to_string()),
    longitude: &starlink_details.longitude.unwrap_or(0.0),
    latitude: &starlink_details.latitude.unwrap_or(0.0),
    height_km: &starlink_details.height_km.unwrap_or(0.0),
    velocity_kms: &starlink_details.velocity_kms.unwrap_or(0.0),
    row_updated: &now,
  };

  let new_starlink_spacetrack_entry = UpdateStarlinkSpacetrack {
    starlink_id: &starlink_details.id,
    ccsds_omm_vers: &starlink_details.spaceTrack.CCSDS_OMM_VERS.clone().unwrap_or("".to_string()),
    comment: &starlink_details.spaceTrack.COMMENT.clone().unwrap_or("".to_string()),
    creation_date: &starlink_details.spaceTrack.CREATION_DATE.clone().unwrap_or("".to_string()),
    originator: &starlink_details.spaceTrack.ORIGINATOR.clone().unwrap_or("".to_string()),
    object_name: &starlink_details.spaceTrack.OBJECT_NAME.clone().unwrap_or("".to_string()),
    object_id: &starlink_details.spaceTrack.OBJECT_ID.clone().unwrap_or("".to_string()),
    center_name: &starlink_details.spaceTrack.CENTER_NAME.clone().unwrap_or("".to_string()),
    ref_frame: &starlink_details.spaceTrack.REF_FRAME.clone().unwrap_or("".to_string()),
    time_system: &starlink_details.spaceTrack.TIME_SYSTEM.clone().unwrap_or("".to_string()),
    mean_element_theory: &starlink_details.spaceTrack.MEAN_ELEMENT_THEORY.clone().unwrap_or("".to_string()),
    epoch: &starlink_details.spaceTrack.EPOCH.clone().unwrap_or("".to_string()),
    mean_motion: &starlink_details.spaceTrack.MEAN_MOTION.unwrap_or(0.0),
    eccentricity: &starlink_details.spaceTrack.ECCENTRICITY.unwrap_or(0.0),
    inclination: &starlink_details.spaceTrack.INCLINATION.unwrap_or(0.0),
    ra_of_asc_node: &starlink_details.spaceTrack.RA_OF_ASC_NODE.unwrap_or(0.0),
    arg_of_pericenter: &starlink_details.spaceTrack.ARG_OF_PERICENTER.unwrap_or(0.0),
    mean_anomaly: &starlink_details.spaceTrack.MEAN_ANOMALY.unwrap_or(0.0),
    ephemeris_type: &starlink_details.spaceTrack.EPHEMERIS_TYPE.unwrap_or(0.0),
    classification_type: &starlink_details.spaceTrack.CLASSIFICATION_TYPE.clone().unwrap_or("".to_string()),
    norad_cat_id: &starlink_details.spaceTrack.NORAD_CAT_ID.unwrap_or(0),
    element_set_no: &starlink_details.spaceTrack.ELEMENT_SET_NO.unwrap_or(0),
    rev_at_epoch: &starlink_details.spaceTrack.REV_AT_EPOCH.unwrap_or(0),
    bstar: &starlink_details.spaceTrack.BSTAR.unwrap_or(0.0),
    mean_motion_dot: &starlink_details.spaceTrack.MEAN_MOTION_DOT.unwrap_or(0.0),
    mean_motion_ddot: &starlink_details.spaceTrack.MEAN_MOTION_DDOT.unwrap_or(0.0),
    semimajor_axis: &starlink_details.spaceTrack.SEMIMAJOR_AXIS.unwrap_or(0.0),
    period: &starlink_details.spaceTrack.PERIOD.unwrap_or(0.0),
    apoapsis: &starlink_details.spaceTrack.APOAPSIS.unwrap_or(0.0),
    periapsis: &starlink_details.spaceTrack.PERIAPSIS.unwrap_or(0.0),
    object_type: &starlink_details.spaceTrack.OBJECT_TYPE.clone().unwrap_or("".to_string()),
    rcs_size: &starlink_details.spaceTrack.RCS_SIZE.clone().unwrap_or("".to_string()),
    country_code: &starlink_details.spaceTrack.COUNTRY_CODE.clone().unwrap_or("".to_string()),
    launch_date: &starlink_details.spaceTrack.LAUNCH_DATE.clone().unwrap_or("".to_string()),
    site: &starlink_details.spaceTrack.SITE.clone().unwrap_or("".to_string()),
    decay_date: &starlink_details.spaceTrack.DECAY_DATE.clone().unwrap_or("".to_string()),
    decayed: &starlink_details.spaceTrack.DECAYED.unwrap_or(0),
    file: &starlink_details.spaceTrack.FILE.unwrap_or(0),
    gp_id: &starlink_details.spaceTrack.GP_ID.unwrap_or(0),
    tle_line0: &starlink_details.spaceTrack.TLE_LINE0.clone().unwrap_or("".to_string()),
    tle_line1: &starlink_details.spaceTrack.TLE_LINE1.clone().unwrap_or("".to_string()),
    tle_line2: &starlink_details.spaceTrack.TLE_LINE2.clone().unwrap_or("".to_string()),
    row_updated: &now,
  };

  diesel::insert_into(starlinks::table)
    .values(&new_starlink_entry)
    .on_conflict(starlinks::starlink_id)
    .do_update()
    .set(&new_starlink_entry)
    .execute(conn)
    .expect("Error saving entry into starlinks table");



  diesel::insert_into(starlink_spacetrack::table)
    .values(&new_starlink_spacetrack_entry)
    .on_conflict(starlink_spacetrack::starlink_id)
    .do_update()
    .set(&new_starlink_spacetrack_entry)
    .execute(conn)
    .expect("Error saving entry into starlink_spacetrack table");
}