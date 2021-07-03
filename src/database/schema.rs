table! {
    capsules (id) {
        id -> Varchar,
        reuse_count -> Nullable<Int4>,
        water_landings -> Nullable<Int4>,
        land_landings -> Nullable<Int4>,
        last_update -> Nullable<Varchar>,
        launches -> Nullable<Array<Text>>,
        serial -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    company_info (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        founder -> Nullable<Varchar>,
        founded -> Nullable<Int4>,
        employees -> Nullable<Int4>,
        vehicles -> Nullable<Int4>,
        launch_sites -> Nullable<Int4>,
        test_sites -> Nullable<Int4>,
        ceo -> Nullable<Varchar>,
        cto -> Nullable<Varchar>,
        coo -> Nullable<Varchar>,
        cto_propulsion -> Nullable<Varchar>,
        valuation -> Nullable<Int8>,
        summary -> Nullable<Text>,
        headquarters_address -> Nullable<Varchar>,
        headquarters_city -> Nullable<Varchar>,
        headquarters_state -> Nullable<Varchar>,
        links_website -> Nullable<Varchar>,
        links_flickr -> Nullable<Varchar>,
        links_twitter -> Nullable<Varchar>,
        links_elon_twitter -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    cores (id) {
        id -> Varchar,
        block -> Nullable<Int4>,
        reuse_count -> Nullable<Int4>,
        rtls_attempts -> Nullable<Int4>,
        rtls_landings -> Nullable<Int4>,
        asds_attempts -> Nullable<Int4>,
        asds_landings -> Nullable<Int4>,
        last_update -> Nullable<Varchar>,
        launches -> Nullable<Array<Text>>,
        serial -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    crew (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        agency -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        wikipedia -> Nullable<Varchar>,
        launches -> Nullable<Array<Text>>,
        status -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    dragons (dragon_id) {
        dragon_id -> Varchar,
        launch_payload_mass_kg -> Nullable<Int4>,
        launch_payload_mass_lbs -> Nullable<Int4>,
        launch_payload_vol_cubic_meters -> Nullable<Int4>,
        launch_payload_vol_cubic_feet -> Nullable<Int4>,
        return_payload_mass_kg -> Nullable<Int4>,
        return_payload_mass_lbs -> Nullable<Int4>,
        return_payload_vol_cubic_meters -> Nullable<Int4>,
        return_payload_vol_cubic_feet -> Nullable<Int4>,
        height_w_trunk_meters -> Nullable<Float8>,
        height_w_trunk_feet -> Nullable<Float8>,
        diameter_meters -> Nullable<Float8>,
        diameter_feet -> Nullable<Float8>,
        first_flight -> Nullable<Varchar>,
        flickr_images -> Nullable<Array<Text>>,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        crew_capacity -> Nullable<Int4>,
        sidewall_angle_deg -> Nullable<Int4>,
        orbit_duration_yr -> Nullable<Int4>,
        dry_mass_kg -> Nullable<Int4>,
        dry_mass_lbs -> Nullable<Int4>,
        wikipedia -> Nullable<Varchar>,
        description -> Nullable<Text>,
        row_updated -> Timestamptz,
    }
}

table! {
    dragons_heat_shield (id) {
        id -> Int4,
        dragon_id -> Varchar,
        material -> Nullable<Varchar>,
        size_meters -> Nullable<Float8>,
        temp_degrees -> Nullable<Int4>,
        dev_partner -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    dragons_pressurized_capsule (id) {
        id -> Int4,
        dragon_id -> Varchar,
        payload_volume_cubic_meters -> Nullable<Int4>,
        payload_volume_cubic_feet -> Nullable<Int4>,
        row_updated -> Timestamptz,
    }
}

table! {
    dragons_thrusters (id) {
        id -> Int4,
        dragon_id -> Varchar,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        amount -> Nullable<Int4>,
        pods -> Nullable<Int4>,
        fuel_1 -> Nullable<Varchar>,
        fuel_2 -> Nullable<Varchar>,
        isp -> Nullable<Int4>,
        thrust_kn -> Nullable<Float8>,
        thrust_lbf -> Nullable<Float8>,
        row_updated -> Timestamptz,
    }
}

table! {
    dragons_trunk (id) {
        id -> Int4,
        dragon_id -> Varchar,
        trunk_volume_cubic_meters -> Nullable<Int4>,
        trunk_volume_cubic_feet -> Nullable<Int4>,
        cargo_solar_array -> Nullable<Int4>,
        cargo_unpressurized_cargo -> Nullable<Bool>,
        row_updated -> Timestamptz,
    }
}

table! {
    roadster_info (id) {
        apoapsis_au -> Nullable<Float8>,
        details -> Nullable<Text>,
        earth_distance_mi -> Nullable<Float8>,
        earth_distance_km -> Nullable<Float8>,
        eccentricity -> Nullable<Float8>,
        epoch_jd -> Nullable<Float8>,
        id -> Varchar,
        inclination -> Nullable<Float8>,
        launch_date_unix -> Nullable<Int4>,
        launch_date_utc -> Nullable<Varchar>,
        launch_mass_kg -> Nullable<Int4>,
        launch_mass_lbs -> Nullable<Int4>,
        longitude -> Nullable<Float8>,
        mars_distance_km -> Nullable<Float8>,
        mars_distance_mi -> Nullable<Float8>,
        name -> Nullable<Varchar>,
        norad_id -> Nullable<Int4>,
        orbit_type -> Nullable<Varchar>,
        periapsis_arg -> Nullable<Float8>,
        periapsis_au -> Nullable<Float8>,
        period_days -> Nullable<Float8>,
        semi_major_axis_au -> Nullable<Float8>,
        speed_kph -> Nullable<Float8>,
        speed_mph -> Nullable<Float8>,
        video -> Nullable<Varchar>,
        wikipedia -> Nullable<Varchar>,
        flickr_images -> Nullable<Array<Text>>,
        row_updated -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    capsules,
    company_info,
    cores,
    crew,
    dragons,
    dragons_heat_shield,
    dragons_pressurized_capsule,
    dragons_thrusters,
    dragons_trunk,
    roadster_info,
);
