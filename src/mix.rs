use actix_web::web::{Data, Path};
use actix_web::{error, get, web, HttpResponse, Responder};

use crate::constants::MAX_MIXES;
use crate::DBPool;

mod actions;
mod models;



/// list the mixes with details (but without food items)
#[get("api/mix/list/")]
pub async fn list_mixes(
    pool: Data<DBPool>
)-> actix_web::Result<impl Responder> {
    // use web::block to offload blocking Diesel queries without blocking server thread
    let mixes =
        web::block(move || {
                // note that obtaining a connection from the pool is also potentially blocking
                let mut conn = pool.get()?;
                    actions::list_mixes(
                        MAX_MIXES, 
                        &mut conn)
            })
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(mixes))
}

/// get a mix details by its id `/mix/{id} (without the food items)`
#[get("api/mix/{id}")]
pub async fn get_mix(
    path: Path<(i32,)>, 
    pool: Data<DBPool>
)-> actix_web::Result<impl Responder> {
    let mix_id = path.0;
    // use web::block to offload blocking Diesel queries without blocking server thread
    let my_mix =
        web::block(move || {
                // note that obtaining a connection from the pool is also potentially blocking
                let mut conn = pool.get()?;
                    actions::get_mix_by_id(
                    mix_id, 
                    &mut conn)
            })
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(my_mix))
}

/// get a mix by its id. `/mix-with-food/{id}`
/// Doesn't include all the mix detail but includes all the food items in the mix
#[get("api/mix-with-food/{id}")]
pub async fn get_mix_with_food(
    path: Path<(i32,)>, 
    pool: Data<DBPool>
)-> actix_web::Result<impl Responder> {
    let mix_id = path.0;
    // use web::block to offload blocking Diesel queries without blocking server thread
    let my_mix =
        web::block(move || {
                // note that obtaining a connection from the pool is also potentially blocking
                let mut conn = pool.get()?;
                    actions::get_mix_by_id_with_food(
                    mix_id, 
                    &mut conn)
            })
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Ok().json(my_mix))
}

