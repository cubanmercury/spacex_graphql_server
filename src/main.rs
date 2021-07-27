#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use std::{env, io};

pub mod database;
pub mod operations;
pub mod queryspacex;

/// simple index handler
#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    // session
    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        counter = count + 1;
    }

    // set counter to session
    session.set("counter", counter)?;

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html")))
}

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

/// response body
async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

/// handler with path parameters like `/user/{name}/`
async fn with_param(req: HttpRequest, web::Path((name,)): web::Path<(String,)>) -> HttpResponse {
    println!("{:?}", req);

    HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("Hello {}!", name))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // queryspacex::get_launches().await;
    let roadster = queryspacex::get_roadster_info().await;

    // println!("ahhhh");
    match roadster {
        Ok(info) => {
            // println!("roadster within main.rs, {:?}", info);
            handle_push_roadster_to_db(info);
        }
        Err(e) => println!("roadster error within main.rs, {:?}", e),
    };

    fn handle_push_roadster_to_db(info: database::models::Roadster) {
        let conn = operations::establish_connection();
        operations::delete_roadster_by_id(&conn, &info);
        // println!("delete previous rows for roadster");
        let push_to_db = operations::create_roadster(&conn, &info);
        // println!("push_to_db: {:?}", push_to_db);
    };

    let company = queryspacex::get_company_info().await;
    // println!("ahhhhhhhhh2");
    match company {
        Ok(info) => {
            // println!("company data within main.rs: {:?}", info);
            handle_push_company_to_db(info)
        }
        Err(e) => println!("Error getting Company JSON data: {:?}", e),
    };

    fn handle_push_company_to_db(info: database::models::Company) {
        let conn = operations::establish_connection();
        operations::delete_company_by_id(&conn, &info);
        operations::create_company(&conn, &info);
    };

    let capsules = queryspacex::get_capsules().await;
    // println!("ahhhhhhhhhhhhh3");
    match capsules {
        Ok(info) => {
            for cap in info {
                println!("capsule: {:?}", cap);
                handle_push_capsules_to_db(cap);
            }
        }
        Err(e) => {
            println!("ERROR GETTING CAPSULES: {:?}", e)
        }
    };

    fn handle_push_capsules_to_db(info: database::models::Capsules) {
        let conn = operations::establish_connection();
        operations::add_capsule(&conn, &info);
    };

    let cores = queryspacex::get_cores().await;
    match cores {
        Ok(info) => {
            for core in info {
                println!("Core data: {:?}", core);
                handle_push_cores_to_db(core);
            }
        }
        Err(e) => {
            println!("Error getting cores: {:?}", e)
        }
    };
    fn handle_push_cores_to_db(info: database::models::Cores) {
        let conn = operations::establish_connection();
        operations::add_core(&conn, &info);
    }

    let crew = queryspacex::get_crew().await;
    match crew {
        Ok(info) => {
            for member in info {
                println!("Crew member: {:?}", member);
                handle_push_crew_to_db(member);
            }
        }
        Err(e) => {
            println!("Error getting crew: {:?}", e)
        }
    };
    fn handle_push_crew_to_db(info: database::models::Crew) {
        let conn = operations::establish_connection();
        operations::add_crew_member(&conn, &info);
    }

    let dragons = queryspacex::get_dragons().await;
    match dragons {
        Ok(info) => {
            for dragon in info {
                println!("Dragon: {:?}", dragon);
                handle_push_dragon_to_db(dragon);
            }
        }
        Err(e) => {
            println!("Error getting Dragons: {:?}", e);
        }
    };
    fn handle_push_dragon_to_db(info: database::models::Dragons) {
        let conn = operations::establish_connection();
        operations::add_dragon(&conn, &info);
    }

    let history = queryspacex::get_history().await;
    match history {
        Ok(info) => {
            for event in info {
                handle_push_history_to_db(event);
            }
        }
        Err(e) => {
            println!("Error getting history: {:?}", e);
        }
    };
    fn handle_push_history_to_db(info: database::models::History) {
        let conn = operations::establish_connection();
        operations::add_history(&conn, &info);
    }

    let landpads = queryspacex::get_landpads().await;
    match landpads {
        Ok(info) => {
            println!("Landpads: {:?}", info);
            for landpad in info {
                handle_push_landpad_to_db(landpad);
            }
        }
        Err(e) => {
            println!("Error getting landpads: {:?}", e);
        }
    };
    fn handle_push_landpad_to_db(info: database::models::Landpads) {
        let conn = operations::establish_connection();
        operations::add_landpad(&conn, &info);
    }

    let launches = queryspacex::get_launches().await;
    match launches {
        Ok(info) => {
            println!("Launches: {:?}", info);
            for launch in info {
                handle_push_launch_to_db(launch);
            }
        }
        Err(e) => {
            println!("Error getting launches: {:?}", e);
        }
    };
    fn handle_push_launch_to_db(launch: database::models::Launches) {
        let conn = operations::establish_connection();
        operations::add_launch(&conn, &launch);
    }

    let launchpads = queryspacex::get_launchpads().await;
    match launchpads {
        Ok(info) => {
            println!("LaunchPads: {:?}", info);
            for launchpad in info {
                handle_push_launchpad_to_db(launchpad);
            }
        }
        Err(e) => {
            println!("Error getting launchpads: {:?}", e);
        }
    };
    fn handle_push_launchpad_to_db(launchpad: database::models::LaunchPads) {
        let conn = operations::establish_connection();
        operations::add_launchpad(&conn, &launchpad);
    }

    let payloads = queryspacex::get_payloads().await;
    match payloads {
        Ok(info) => {
            println!("Payloads: {:?}", info);
            for payload in info {
                handle_push_payload_to_db(payload);
            }
        }
        Err(e) => {
            println!("Error getting payloads: {:?}", e);
        }
    };
    fn handle_push_payload_to_db(payload: database::models::Payloads) {
        let conn = operations::establish_connection();
        operations::add_payload(&conn, &payload);
    }

    let rockets = queryspacex::get_rockets().await;
    match rockets {
        Ok(info) => {
            println!("Rockets: {:?}", info);
            for rocket in info {
                handle_push_rocket_to_db(rocket);
            }
        }
        Err(e) => {
            println!("Error getting rockets: {:?}", e);
        }
    };
    fn handle_push_rocket_to_db(rocket: database::models::Rockets) {
        let conn = operations::establish_connection();
        operations::add_rocket(&conn, &rocket);
    };

    HttpServer::new(|| {
        App::new()
            .app_data(web::PayloadConfig::new(1000000 * 250))
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register simple route, handle all methods
            .service(welcome)
            // with path parameters
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))
            // async response body
            .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
            .service(
                web::resource("/test").to(|req: HttpRequest| match *req.method() {
                    Method::GET => HttpResponse::Ok(),
                    Method::POST => HttpResponse::MethodNotAllowed(),
                    _ => HttpResponse::NotFound(),
                }),
            )
            .service(web::resource("/error").to(|| async {
                error::InternalError::new(
                    io::Error::new(io::ErrorKind::Other, "test"),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            }))
            // static files
            .service(fs::Files::new("/static", "static").show_files_listing())
            // redirect
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {
                println!("{:?}", req);
                HttpResponse::Found()
                    .header(header::LOCATION, "static/welcome.html")
                    .finish()
            })))
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
