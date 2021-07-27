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
        id -> Varchar,
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
    history (id) {
        id -> Varchar,
        title -> Nullable<Varchar>,
        event_date_utc -> Nullable<Varchar>,
        event_date_unix -> Nullable<Int4>,
        details -> Nullable<Text>,
        links_article -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    landpads (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        full_name -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        locality -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        landing_attempts -> Nullable<Int4>,
        landing_successes -> Nullable<Int4>,
        wikipedia -> Nullable<Varchar>,
        details -> Nullable<Text>,
        launches -> Nullable<Array<Text>>,
        row_updated -> Timestamptz,
    }
}

table! {
    launch_cores (id) {
        id -> Varchar,
        launch_id -> Varchar,
        core -> Nullable<Varchar>,
        flight -> Nullable<Int4>,
        gridfins -> Nullable<Bool>,
        legs -> Nullable<Bool>,
        reused -> Nullable<Bool>,
        landing_attempt -> Nullable<Bool>,
        landing_success -> Nullable<Bool>,
        landing_type -> Nullable<Varchar>,
        landpad -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    launch_failures (id) {
        id -> Varchar,
        launch_id -> Varchar,
        time -> Nullable<Int4>,
        altitude -> Nullable<Int4>,
        reason -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    launch_fairings (id) {
        id -> Int4,
        launch_id -> Varchar,
        reused -> Nullable<Bool>,
        recovery_attempt -> Nullable<Bool>,
        recovered -> Nullable<Bool>,
        ships -> Nullable<Array<Text>>,
        row_updated -> Timestamptz,
    }
}

table! {
    launch_links (id) {
        id -> Int4,
        launch_id -> Varchar,
        patch_small -> Nullable<Varchar>,
        patch_large -> Nullable<Varchar>,
        reddit_campaign -> Nullable<Varchar>,
        reddit_launch -> Nullable<Varchar>,
        reddit_media -> Nullable<Varchar>,
        reddit_recovery -> Nullable<Varchar>,
        flickr_small -> Nullable<Array<Text>>,
        flickr_original -> Nullable<Array<Text>>,
        presskit -> Nullable<Varchar>,
        webcast -> Nullable<Varchar>,
        youtube_id -> Nullable<Varchar>,
        article -> Nullable<Varchar>,
        wikipedia -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    launches (launch_id) {
        launch_id -> Varchar,
        name -> Nullable<Varchar>,
        static_fire_date_utc -> Nullable<Varchar>,
        static_fire_date_unix -> Nullable<Int4>,
        tdb -> Nullable<Bool>,
        net -> Nullable<Bool>,
        window_number -> Nullable<Int4>,
        rocket -> Nullable<Varchar>,
        success -> Nullable<Bool>,
        details -> Nullable<Text>,
        crew -> Nullable<Array<Text>>,
        ships -> Nullable<Array<Text>>,
        capsules -> Nullable<Array<Text>>,
        payloads -> Nullable<Array<Text>>,
        launchpad -> Nullable<Varchar>,
        auto_update -> Nullable<Bool>,
        flight_number -> Nullable<Int4>,
        date_utc -> Nullable<Varchar>,
        date_unix -> Nullable<Int4>,
        date_local -> Nullable<Varchar>,
        date_precision -> Nullable<Varchar>,
        upcoming -> Nullable<Bool>,
        row_updated -> Timestamptz,
    }
}

table! {
    launchpads (id) {
        id -> Varchar,
        name -> Nullable<Varchar>,
        full_name -> Nullable<Varchar>,
        locality -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        timezone -> Nullable<Varchar>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        launch_attempts -> Nullable<Int4>,
        launch_successes -> Nullable<Int4>,
        rockets -> Nullable<Array<Text>>,
        launches -> Nullable<Array<Text>>,
        status -> Nullable<Varchar>,
        row_updated -> Timestamptz,
    }
}

table! {
    payload_dragon (id) {
        id -> Int4,
        payload_id -> Varchar,
        capsule -> Nullable<Varchar>,
        mass_returned_kg -> Nullable<Float8>,
        mass_returned_lbs -> Nullable<Float8>,
        flight_time_sec -> Nullable<Int8>,
        manifest -> Nullable<Varchar>,
        water_landing -> Nullable<Bool>,
        land_landing -> Nullable<Bool>,
        row_updated -> Timestamptz,
    }
}

table! {
    payloads (payload_id) {
        payload_id -> Varchar,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        reused -> Nullable<Bool>,
        launch -> Nullable<Varchar>,
        customers -> Nullable<Array<Text>>,
        norad_ids -> Nullable<Array<Int4>>,
        nationalities -> Nullable<Array<Text>>,
        manufacturers -> Nullable<Array<Text>>,
        mass_kg -> Nullable<Float8>,
        mass_lbs -> Nullable<Float8>,
        orbit -> Nullable<Varchar>,
        reference_system -> Nullable<Varchar>,
        regime -> Nullable<Varchar>,
        longitude -> Nullable<Float8>,
        semi_major_axis_km -> Nullable<Float8>,
        eccentricity -> Nullable<Float8>,
        periapsis_km -> Nullable<Float8>,
        apoapsis_km -> Nullable<Float8>,
        inclination_deg -> Nullable<Float8>,
        period_min -> Nullable<Float8>,
        lifespan_years -> Nullable<Float8>,
        epoch -> Nullable<Varchar>,
        mean_motion -> Nullable<Float8>,
        raan -> Nullable<Float8>,
        arg_of_pericenter -> Nullable<Float8>,
        mean_anomaly -> Nullable<Float8>,
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

table! {
    rocket_engine (id) {
        id -> Int4,
        rocket_id -> Varchar,
        isp_sea_level -> Nullable<Int4>,
        isp_vacuum -> Nullable<Int4>,
        thrust_sea_level_kn -> Nullable<Float8>,
        thrust_sea_level_lbf -> Nullable<Float8>,
        thrust_vacuum_kn -> Nullable<Float8>,
        thrust_vacuum_lbf -> Nullable<Float8>,
        number -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        version -> Nullable<Varchar>,
        layout -> Nullable<Varchar>,
        engine_loss_max -> Nullable<Int4>,
        propellant_1 -> Nullable<Varchar>,
        propellant_2 -> Nullable<Varchar>,
        thrust_to_weight -> Nullable<Float8>,
        row_updated -> Timestamptz,
    }
}

table! {
    rocket_first_stage (id) {
        id -> Int4,
        rocket_id -> Varchar,
        thrust_sea_level_kn -> Nullable<Float8>,
        thrust_sea_level_lbf -> Nullable<Float8>,
        thrust_vacuum_kn -> Nullable<Float8>,
        thrust_vacuum_lbf -> Nullable<Float8>,
        reusable -> Nullable<Bool>,
        engines -> Nullable<Int4>,
        fuel_amount_tons -> Nullable<Float8>,
        burn_time_sec -> Nullable<Int4>,
        row_updated -> Timestamptz,
    }
}

table! {
    rocket_payload_weights (id) {
        id -> Varchar,
        rocket_id -> Varchar,
        payload_id -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        kg -> Nullable<Int4>,
        lb -> Nullable<Int4>,
        row_updated -> Timestamptz,
    }
}

table! {
    rocket_second_stage (id) {
        id -> Int4,
        rocket_id -> Varchar,
        thrust_kn -> Nullable<Float8>,
        thrust_lbf -> Nullable<Float8>,
        payloads_composite_fairing_height_meters -> Nullable<Float8>,
        payloads_composite_fairing_height_feet -> Nullable<Float8>,
        payloads_composite_fairing_diameter_meters -> Nullable<Float8>,
        payloads_composite_fairing_diameter_feet -> Nullable<Float8>,
        payloads_option_1 -> Nullable<Varchar>,
        reusable -> Nullable<Bool>,
        engines -> Nullable<Int4>,
        fuel_amount_tons -> Nullable<Float8>,
        burn_time_sec -> Nullable<Int4>,
        row_updated -> Timestamptz,
    }
}

table! {
    rockets (rocket_id) {
        rocket_id -> Varchar,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Nullable<Varchar>,
        active -> Nullable<Bool>,
        stages -> Nullable<Int4>,
        boosters -> Nullable<Int4>,
        cost_per_launch -> Nullable<Int8>,
        success_rate_pct -> Nullable<Int4>,
        first_flight -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        company -> Nullable<Varchar>,
        wikipedia -> Nullable<Varchar>,
        description -> Nullable<Text>,
        height_meters -> Nullable<Float8>,
        height_feet -> Nullable<Float8>,
        diameter_meters -> Nullable<Float8>,
        diameter_feet -> Nullable<Float8>,
        mass_kg -> Nullable<Int4>,
        mass_lbs -> Nullable<Int4>,
        landing_legs_number -> Nullable<Int4>,
        landing_legs_material -> Nullable<Varchar>,
        flickr_images -> Nullable<Array<Text>>,
        row_updated -> Timestamptz,
    }
}

joinable!(payload_dragon -> payloads (payload_id));

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
    history,
    landpads,
    launch_cores,
    launch_failures,
    launch_fairings,
    launch_links,
    launches,
    launchpads,
    payload_dragon,
    payloads,
    roadster_info,
    rocket_engine,
    rocket_first_stage,
    rocket_payload_weights,
    rocket_second_stage,
    rockets,
);
