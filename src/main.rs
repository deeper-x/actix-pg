use actix_web::{web, App, HttpServer};
use confik::{Configuration as _, EnvSource};
use dotenvy::dotenv;
use tokio_postgres::NoTls;

use crate::settings::config::ServerConfig;

mod settings {
    pub mod config;
    pub mod errors;
}
mod db {
    pub mod dml;
    pub mod models;
}

mod server {
    pub mod router;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // build server configuration via config::Configuration implementations
    let config = ServerConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    // database setup, configuring pool
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    // http server instance setup, linking to `/users' controller its routes (add & get)
    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/users")
                .route(web::post().to(server::router::add_user))
                .route(web::get().to(server::router::get_users)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();

    println!("Server running at http://{}/", config.server_addr);

    server.await
}
