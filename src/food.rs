use actix_web::web::{Data, Json, Path};
use actix_web::{error, get, post, delete, put, web, HttpResponse, Responder};

use serde::Deserialize;

use crate::constants::{CONNECTION_POOL_ERROR, MAX_FOOD_ITEMS};
use crate::DBPool;

mod actions;
mod models;



/// get a food item by its id `/food/{id}`
#[get("api/food/{id}")]
pub async fn get(
    path: Path<(i32,)>, 
    pool: Data<DBPool>
)-> actix_web::Result<impl Responder> {
    let food_id = path.0;
    // use web::block to offload blocking Diesel queries without blocking server thread
    let food_item =
        web::block(move || {
                // note that obtaining a connection from the pool is also potentially blocking
                let mut conn = pool.get()?;
                    actions::get_food_item_by_id(
                    food_id, 
                    &mut conn)
            })
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

            Ok(match food_item {
                // food item was found; return 200 response with JSON formatted user object
                Some(fi) => HttpResponse::Ok().json(fi),
        
                // food item was not found; return 404 response with error message
                None => HttpResponse::NotFound().body(format!("No food item found with id: {food_id}")),
            })
}


/// list all food items
#[get("api/food")]
pub async fn list(pool: Data<DBPool>
)-> actix_web::Result<impl Responder>  {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("called get without name");
    let food_items = 
        web::block(
            move || 
                actions::list_food_items(
                    MAX_FOOD_ITEMS, 
                    &mut conn)
            )
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(food_items))
}


#[derive(Debug, Deserialize)]
pub struct FoodSearch {
   name: String
}

/// search food items by name
#[get("api/food/search/")]
pub async fn search(
    query_params: web::Query<FoodSearch>, 
    pool: Data<DBPool>
) -> actix_web::Result<impl Responder> {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("name searched={}", query_params.name);
    let food_items = 
        web::block(
            move || 
                actions::find_food_items_by_name(
                    MAX_FOOD_ITEMS,
                    &query_params.name, 
                    &mut conn)
            )
            .await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(food_items))
}


/// create a food item `/food`
#[post("api/food")]
pub async fn create(new_food: Json<models::NewFood>, pool: Data<DBPool>
) -> actix_web::Result<impl Responder> {

    let food = match new_food.to_food(){
        Some(f) => f,
        _ => return Ok(HttpResponse::BadRequest().body(format!("A least one field is missing")))
    };

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    
    let food_item = 
        web::block(
            move || 
                actions::create_food_item(
                    food, 
                    &mut conn)
            ).await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

    // user was added successfully; return 201 response with new user info
    Ok(HttpResponse::Created().json(food_item))
}

/// delete a food item by its id `/food/{id}`
#[delete("api/food/{id}")]
pub async fn delete(path: Path<(i32,)>, pool: Data<DBPool>) 
-> actix_web::Result<impl Responder> {
    // in any case return status 204
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let _ = web::block(
        move || 
            actions::delete_food_item(
                path.0, &mut conn
            )
        ).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent())
}


/// update a food item by its id `/food/{id}`
/// overwrite all fields
/// TODO : a patch would be nice too
#[put("api/food/{id}")]
pub async fn update(path: Path<(i32,)>, food_req: Json<models::NewFood>, pool: Data<DBPool>) 
-> actix_web::Result<impl Responder> {

    let food = match food_req.to_food(){
        Some(f) => f,
        _ => return Ok(HttpResponse::BadRequest().body(format!("A least one field is missing")))
    };

    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    let food_item = 
        web::block(
            move || 
                actions::update_food_item(
                    path.0,
                    food,
                    &mut conn)
            ).await?
            // map diesel query errors to a 500 error response
            .map_err(error::ErrorInternalServerError)?;

            Ok(HttpResponse::Created().json(food_item))
}
