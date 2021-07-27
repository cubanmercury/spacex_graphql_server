use actix_web::dev::Decompress;
use actix_web::dev::Payload;
use awc::error::SendRequestError;
use awc::Client;
use awc::ClientResponse;
// use serde_derive::Deserialize;

use crate::database::models::Company;
use crate::database::models::Roadster;
use crate::database::models::*;

pub async fn get_client(
    endpoint: &str,
) -> Result<ClientResponse<Decompress<Payload>>, SendRequestError> {
    let mut error_found: bool = false;
    let client = Client::default();
    let resp = client
        .get(endpoint)
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await;

    let client_resp = resp.and_then(|r| {
        match r.status().is_success() {
            true => println!("Client request success for endpoint: {:?}", endpoint),
            false => error_found = true,
        }

        if error_found == false {
            Ok(r)
        } else {
            Err(awc::error::SendRequestError::Url(
                awc::error::InvalidUrl::UnknownScheme,
            ))
        }
    });

    return client_resp;
}

// Query SpaceX_API for roadster_info
pub async fn get_roadster_info() -> Result<Roadster, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/roadster");
    let response = get_client(&endpoint).await;

    let roadster_info: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_roadster_info")
        .json::<Roadster>()
        .await
    {
        Ok(v) => {
            roadster_info = Ok(v);
        }
        Err(e) => {
            println!("Error getting Roadster JSON data: {:?}", e);
            roadster_info = Err(e);
        }
    };

    return roadster_info;
}

// Query SpaceX_API for Company_info
pub async fn get_company_info() -> Result<Company, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/company");
    let response = get_client(&endpoint).await;

    let company_info: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_company_info")
        .json::<Company>()
        .await
    {
        Ok(v) => company_info = Ok(v),
        Err(e) => {
            println!("Error getting Company JSON data: {:?}", e);
            company_info = Err(e);
        }
    };

    return company_info;
}

// Query SpaceX_API for Capsules info
pub async fn get_capsules() -> Result<Vec<Capsules>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/capsules");
    let response = get_client(&endpoint).await;

    let capsules: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_capsules")
        .json::<Vec<Capsules>>()
        .await
    {
        Ok(v) => capsules = Ok(v),
        Err(e) => {
            println!("Error getting Capsule JSON data: {:?}", e);
            capsules = Err(e);
        }
    };

    return capsules;
}

// Query SpaceX_API for Cores info
pub async fn get_cores() -> Result<Vec<Cores>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/cores");
    let response = get_client(&endpoint).await;

    let cores: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_cores()")
        .json::<Vec<Cores>>()
        .await
    {
        Ok(v) => cores = Ok(v),
        Err(e) => {
            println!("Error getting Cores data: {:?}", e);
            cores = Err(e)
        }
    };

    return cores;
}

// Query SpaceX_API for Crew info
pub async fn get_crew() -> Result<Vec<Crew>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/crew");
    let response = get_client(&endpoint).await;

    let crew: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_crew()")
        .json::<Vec<Crew>>()
        .await
    {
        Ok(v) => crew = Ok(v),
        Err(e) => {
            println!("Error getting Crew data: {:?}", e);
            crew = Err(e)
        }
    };

    return crew;
}

// Query SpaceX_API for Dragons info
pub async fn get_dragons() -> Result<Vec<Dragons>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/dragons");
    let response = get_client(&endpoint).await;

    let dragons: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_dragons()")
        .json::<Vec<Dragons>>()
        .await
    {
        Ok(v) => {
            dragons = Ok(v);
        }
        Err(e) => {
            println!("Error getting Dragon data: {:?}", e);
            dragons = Err(e);
        }
    };

    return dragons;
}

// Query SpaceX_API for Hisoric Events info
pub async fn get_history() -> Result<Vec<History>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/history");
    let response = get_client(&endpoint).await;

    let history: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_history()")
        .json::<Vec<History>>()
        .await
    {
        Ok(v) => history = Ok(v),
        Err(e) => {
            println!("Error getting History data: {:?}", e);
            history = Err(e);
        }
    };

    return history;
}

// Query SpaceX_API for Landing Pads info
pub async fn get_landpads() -> Result<Vec<Landpads>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/landpads");
    let response = get_client(&endpoint).await;

    let landpad: Result<_,_>;

    match response
        .expect("Error getting JSON data for get_landpads()")
        .json::<Vec<Landpads>>()
        .await
    {
        Ok(v) => landpad = Ok(v),
        Err(e) => {
            println!("Error getting Landpad data: {:?}", e);
            landpad = Err(e);
        }
    };

    return landpad;
}

// Query SpaceX_API for Landing Pads info
pub async fn get_launches() -> Result<Vec<Launches>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/launches");
    let response = get_client(&endpoint).await;

    let launch: Result<_,_>;

    match response
        .expect("Error getting JSON data for get_lauches()")
        .json::<Vec<Launches>>()
        .limit(750000) //750kb limit on payload size, default max-size = 64kb
        .await
    {
        Ok(v) => launch = Ok(v),
        Err(e) => {
            println!("Error getting Launch data: {:?}", e);
            launch = Err(e);
        }
    };

    return launch;
}

// Query SpaceX_API for Launch Pad info
pub async fn get_launchpads() -> Result<Vec<LaunchPads>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/launchpads");
    let response = get_client(&endpoint).await;

    let launchpad: Result<_,_>;

    match response
        .expect("Error getting JSON data for get_launchpads()")
        .json::<Vec<LaunchPads>>()
        .await
    {
        Ok(v) => launchpad = Ok(v),
        Err(e) => {
            println!("Error getting Launchpad data: {:?}", e);
            launchpad = Err(e);
        }
    };

    return launchpad;
}

// Query SpaceX_API for Payloads info
pub async fn get_payloads() -> Result<Vec<Payloads>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/payloads");
    let response = get_client(&endpoint).await;

    let payload: Result<_,_>;

    match response
        .expect("Error getting JSON data for get_payloads()")
        .json::<Vec<Payloads>>()
        .limit(750000)
        .await
    {
        Ok(v) => payload = Ok(v),
        Err(e) => {
            println!("Error getting Payload data: {:?}", e);
            payload = Err(e);
        }
    };

    return payload;
}

// Query SpaceX_API for Rockets info
pub async fn get_rockets() -> Result<Vec<Rockets>, actix_web::client::JsonPayloadError> {
    let endpoint = String::from("https://api.spacexdata.com/v4/rockets");
    let response = get_client(&endpoint).await;

    let rocket: Result<_,_>;

    match response
        .expect("Error getting JSON data for get_rockets()")
        .json::<Vec<Rockets>>()
        .await
    {
        Ok(v) => rocket = Ok(v),
        Err(e) => {
            println!("Error getting Rocket data: {:?}", e);
            rocket = Err(e);
        }
    };

    return rocket;
}