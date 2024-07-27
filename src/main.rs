use dotenvy::dotenv;
use std::env;
use std::io::Result;

use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod schema;
mod food;
mod mix;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // set up database connection pool
    let database_url = env::var("POSTGRESQL_ADDON_URI").expect("POSTGRESQL_ADDON_URI");
    let bin_addr = env::var("BIND_ADDR").expect("BIND_ADDR");
    let max_db_connections: u32 = env::var("MAX_DB_CONNECTIONS").expect("MAX_DB_CONNECTIONS").parse().unwrap();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(max_db_connections)
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(food::get)
            .service(food::search)
            .service(food::search_i18n)
            // waiting for authentication implementation
            //.service(food::create)
            //.service(food::delete)
            //.service(food::update)
            .service(mix::get_mix)
            .service(mix::get_mix_with_food)
            .service(mix::get_mix_with_food_i18n)
            .service(mix::list_mixes)
    })
    .bind(bin_addr)?
    .run()
    .await
}