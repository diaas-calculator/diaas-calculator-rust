use dotenvy::dotenv;
use std::env;
use std::io::Result;

use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod response;
mod schema;
mod food;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    dotenv().ok();
    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(food::list)
            .service(food::get)
            //.service(food::create)
            //.service(food::delete)
            .service(food::search)
            //.service(food::update)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}