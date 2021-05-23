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
    }
}

allow_tables_to_appear_in_same_query!(
    company_info,
    roadster_info,
);
 