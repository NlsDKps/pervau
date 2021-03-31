#[macro_use] extern crate rocket;

use log::debug;
use pervau::{
    controller::database::local_conn_string,
    view::{
        api,
        html
    }
};
use rocket_contrib::{
    serve::StaticFiles,
    templates::Template
};

fn main() {
    env_logger::init();

    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    debug!("Running server {}, version {}", name, version);

    let html_routes = routes![
        html::bloodpressure::list,
        html::bloodpressure::add
    ];

    let api_routes = routes![
        api::bloodpressure::add,
        api::bloodpressure::get_all,
        api::bloodpressure::get_by_id,
        api::bloodpressure::delete_by_id,
    ];

    let db_url = match local_conn_string() {
        Some(db_url) => db_url,
        None => return
    };

    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", html_routes)
        .mount("/api", api_routes)
        .manage(db_url)
        .attach(Template::fairing())
        .launch();
}
