use actix_web::dev::Decompress;
use actix_web::dev::Payload;
use awc::error::SendRequestError;
use awc::Client;
use awc::ClientResponse;
use serde_derive::Deserialize;

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

pub async fn get_launches() {
    let client = Client::default();

    let response = client
        .get("https://api.spacexdata.com/v4/roadster")
        .header("User-Agent", "actix-web/3.0")
        .send()
        .await;

    // match response.status() {
    //   StatusCode::Ok => {
    //     println!("status success");
    //     let k = response.body().await.unwrap();
    //     println!("unwrapped body: {:?}", &k);
    //   }
    //   _ => println!("status failed")
    // }

    let r = response.and_then(|resp| {
        println!("Resp: {:?}", resp);
        // resp.json().and_then(move |bytes| {
        //   let s = std::str::from_utf8(&bytes).expect("utf8 parse error");
        //   println!("html: {:?}", s);
        // });
        // println!("body: {:?}", resp.json());

        // resp.json().and_then(move |j| {
        //    match j {
        //      Ok(body) => body
        //    }
        // });

        // let b = match r {
        //     JsonBody => println!("JSONBody:" ),
        //     // JsonBody::JsonPayloadError(Error) => {
        //     //   panic!("response json body not found: {:?}", Error)
        //     // }
        //     _ => println!("it was some other shit, who knows")
        // };

        // println!("body: {:?}", b);

        match resp.status().as_u16() {
            200 => println!("resp works"),
            301 => println!("resp redirects"),
            404 => println!("resp not found"),
            _ => println!("resp has unknown status code"),
        };

        match resp.status().is_success() {
            true => println!("resp is successful"),
            false => println!("resp is unsuccessful"),
        };
        match resp.status().is_redirection() {
            true => println!("resp is a redirect"),
            false => println!("resp is not a redirect"),
        };

        // resp.body().map_err(|_| ()).and_then(|bytes| {
        //     println!("{:?}", bytes);
        //     Ok(())
        // });

        // match body {
        //     awc::MessageBody => println!("message body matched"),
        //     _ => println!("message body not matched")
        // };

        Ok(resp)
    });

    // let n = match r {
    //     Ok(resp) => println!("resp matched: {:?}", resp),
    //     Err(e) => println!("resp match error: {:?}", e)
    // };

    #[derive(Deserialize, Debug)]
    pub struct Roadster {
        flickr_images: [String; 4],
        name: String,
        launch_date_utc: String,
        launch_date_unix: u32,
        launch_mass_kg: u32,
        launch_mass_lbs: u32,
        norad_id: u32,
        epoch_jd: f64,
        orbit_type: String,
        apoapsis_au: f64,
        periapsis_au: f64,
        semi_major_axis_au: f64,
        eccentricity: f64,
        inclination: f64,
        longitude: f64,
        periapsis_arg: f64,
        period_days: f64,
        speed_kph: f64,
        speed_mph: f64,
        earth_distance_km: f64,
        earth_distance_mi: f64,
        mars_distance_km: f64,
        mars_distance_mi: f64,
        wikipedia: String,
        video: String,
        details: String,
        id: String,
    }

    match r.expect("Error here").json::<Roadster>().await {
        Ok(v) => println!("k: {:?}", v),
        Err(e) => println!("k not found: {:?}", e),
    }
    // match r.expect("Error deserialising json response").json::<serde_json::Value>().await {
    //     Ok(v) => println!("k: {:?}", v),
    //     Err(e) => println!("k not found: {:?}", e)
    // }

    // k.body().and_then(|bytes| {
    //     println!("bytes of body: {:?} ", bytes);
    //     Ok(())
    // });

    // println!("Response here: {:?}", response);
}

#[derive(Deserialize, Debug)]
pub struct Roadster {
    flickr_images: [String; 4],
    name: String,
    launch_date_utc: String,
    launch_date_unix: u32,
    launch_mass_kg: u32,
    launch_mass_lbs: u32,
    norad_id: u32,
    epoch_jd: f64,
    orbit_type: String,
    apoapsis_au: f64,
    periapsis_au: f64,
    semi_major_axis_au: f64,
    eccentricity: f64,
    inclination: f64,
    longitude: f64,
    periapsis_arg: f64,
    period_days: f64,
    speed_kph: f64,
    speed_mph: f64,
    earth_distance_km: f64,
    earth_distance_mi: f64,
    mars_distance_km: f64,
    mars_distance_mi: f64,
    wikipedia: String,
    video: String,
    details: String,
    id: String,
}

pub async fn get_roadster_info() -> Result<Roadster, actix_web::client::JsonPayloadError> {
    let endpoint: &str = "https://api.spacexdata.com/v4/roadster";
    let response = get_client(&endpoint).await;

    let roadster_info: Result<_, _>;

    match response
        .expect("Error getting JSON data for get_roadster")
        .json::<Roadster>()
        .await
    {
        Ok(v) => {
            println!("Roadster JSON data: {:?}", v);
            roadster_info = Ok(v);
        }
        Err(e) => {
            println!("Error getting Roadster JSON data: {:?}", e);
            roadster_info = Err(e);
        }
    };

    return roadster_info;
}
