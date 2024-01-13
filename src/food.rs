use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, delete, put, web, HttpResponse};
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl, TextExpressionMethods};
use serde::{Deserialize, Serialize};

use crate::constants::{APPLICATION_JSON, CONNECTION_POOL_ERROR, MAX_FOOD_ITEMS};
use crate::response::Response;
use crate::{DBPool, DBPooledConnection};

use super::schema::food;
use diesel::query_dsl::methods::{FilterDsl, LimitDsl};

pub type FoodItems = Response<Food>;

//TODO gérer cas reference_details et comment null
#[derive(Debug, Deserialize, Serialize)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub protein_content: f32,
    pub food_type: String,
    pub score_type: String,
    pub protein_content_cooked_state: String,
    pub diaas_cooked_state: String,
    pub histidine_score: f32,
    pub isoleucine_score: f32,
    pub leucine_score: f32,
    pub lysine_score: f32,
    pub saa_score: f32,
    pub aaa_score: f32,
    pub threonine_score: f32,
    pub tryptophane_score: f32,
    pub valine_score: f32,
    pub reference_link: String,
    pub reference_details: String,
    pub comment: String
}

impl Food {
    pub fn new(
            name: String, 
            protein_content: f32,
            food_type: String, 
            score_type: String,
            protein_content_cooked_state: String,
            diaas_cooked_state: String,
            histidine_score: f32,
            isoleucine_score: f32,
            leucine_score: f32,
            lysine_score: f32,
            saa_score: f32,
            aaa_score: f32,
            threonine_score: f32,
            tryptophane_score: f32,
            valine_score: f32,
            reference_link: String,
            reference_details: String,
            comment: String
            ) -> Self {
        Self {
            // will be set by the database
            id: 0,
            name,
            protein_content,
            food_type,
            score_type,
            protein_content_cooked_state,
            diaas_cooked_state,
            histidine_score,
            isoleucine_score,
            leucine_score,
            lysine_score,
            saa_score,
            aaa_score,
            threonine_score,
            tryptophane_score,
            valine_score,
            reference_link,
            reference_details,
            comment
        }
    }


    pub fn to_food_db_for_insert(&self) -> FoodDBForInsert {
        FoodDBForInsert {
            name: self.name.clone(),
            protein_content: self.protein_content,
            food_type: self.food_type.clone(),
            score_type: self.score_type.clone(),
            protein_content_cooked_state: self.protein_content_cooked_state.clone(),
            diaas_cooked_state: self.diaas_cooked_state.clone(),
            histidine_score: self.histidine_score,
            isoleucine_score: self.isoleucine_score,
            leucine_score: self.leucine_score,
            lysine_score: self.lysine_score,
            saa_score: self.saa_score,
            aaa_score: self.aaa_score,
            threonine_score: self.threonine_score,
            tryptophane_score: self.tryptophane_score,
            valine_score: self.valine_score,
            reference_link: self.reference_link.clone(),
            reference_details: self.reference_details.clone(),
            comment: self.comment.clone()
        }
    }
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = food)]
pub struct FoodDB {
    pub id: i32,
    pub name: String,
    pub protein_content: f32,
    pub food_type: String,
    pub score_type: String,
    pub protein_content_cooked_state: String,
    pub diaas_cooked_state: String,
    pub histidine_score: f32,
    pub isoleucine_score: f32,
    pub leucine_score: f32,
    pub lysine_score: f32,
    pub saa_score: f32,
    pub aaa_score: f32,
    pub threonine_score: f32,
    pub tryptophane_score: f32,
    pub valine_score: f32,
    pub reference_link: String,
    pub reference_details: String,
    pub comment: String
}

// FoodDB without the id. Required for creating a new object before knowing the id that will be attributed by the database
#[derive(Insertable)]
#[diesel(table_name = food)]
pub struct FoodDBForInsert {
    pub name: String,
    pub protein_content: f32,
    pub food_type: String,
    pub score_type: String,
    pub protein_content_cooked_state: String,
    pub diaas_cooked_state: String,
    pub histidine_score: f32,
    pub isoleucine_score: f32,
    pub leucine_score: f32,
    pub lysine_score: f32,
    pub saa_score: f32,
    pub aaa_score: f32,
    pub threonine_score: f32,
    pub tryptophane_score: f32,
    pub valine_score: f32,
    pub reference_link: String,
    pub reference_details: String,
    pub comment: String
}

impl FoodDB {
    fn to_food(&self) -> Food {
        Food {
            id: self.id,
            name: self.name.clone(),
            protein_content: self.protein_content,
            food_type: self.food_type.clone(),
            score_type: self.score_type.clone(),
            protein_content_cooked_state: self.protein_content_cooked_state.clone(),
            diaas_cooked_state: self.diaas_cooked_state.clone(),
            histidine_score: self.histidine_score,
            isoleucine_score: self.isoleucine_score,
            leucine_score: self.leucine_score,
            lysine_score: self.lysine_score,
            saa_score: self.saa_score,
            aaa_score: self.aaa_score,
            threonine_score: self.threonine_score,
            tryptophane_score: self.tryptophane_score,
            valine_score: self.valine_score,
            reference_link: self.reference_link.clone(),
            reference_details: self.reference_details.clone(),
            comment: self.comment.clone()
        }
    }
}

// For creating (and updating) a food (without an id)
#[derive(Debug, Deserialize, Serialize)]
pub struct FoodRequest {
    pub name: Option<String>,
    pub protein_content: Option<f32>,
    pub food_type: Option<String>,
    pub score_type: Option<String>,
    pub protein_content_cooked_state: Option<String>,
    pub diaas_cooked_state: Option<String>,
    pub histidine_score: Option<f32>,
    pub isoleucine_score: Option<f32>,
    pub leucine_score: Option<f32>,
    pub lysine_score: Option<f32>,
    pub saa_score: Option<f32>,
    pub aaa_score: Option<f32>,
    pub threonine_score: Option<f32>,
    pub tryptophane_score: Option<f32>,
    pub valine_score: Option<f32>,
    pub reference_link: Option<String>,
    pub reference_details: Option<String>,
    pub comment: Option<String>
}

impl FoodRequest {
    pub fn to_food(&self) -> Option<Food> {
        match (&self.name, self.protein_content, &self.food_type, &self.score_type, &self.protein_content_cooked_state, &self.diaas_cooked_state, self.histidine_score, self.isoleucine_score, self.leucine_score, self.lysine_score, self.saa_score, self.aaa_score, self.threonine_score, self.tryptophane_score, self.valine_score, &self.reference_link, &self.reference_details, &self.comment) {
            (Some(name), Some(protein_content), Some(food_type), Some(score_type), Some(protein_content_cooked_state), Some(diaas_cooked_state), Some(histidine_score), Some(isoleucine_score), Some(leucine_score), Some(lysine_score), Some(saa_score), Some(aaa_score), Some(threonine_score), Some(tryptophane_score), Some(valine_score), Some(reference_link), Some(reference_details), Some(comment)) =>  
                Some(
                        Food::new(
                            name.to_string(),
                            protein_content,
                            food_type.to_string(),
                            score_type.to_string(),
                            protein_content_cooked_state.to_string(),
                            diaas_cooked_state.to_string(),
                            histidine_score,
                            isoleucine_score,
                            leucine_score,
                            lysine_score,
                            saa_score,
                            aaa_score,
                            threonine_score,
                            tryptophane_score,
                            valine_score,
                            reference_link.to_string(),
                            reference_details.to_string(),
                            comment.to_string()
                        )
                    ),
            _ => None
        }
    }
}



fn list_food_items(max_items: i64, conn: &mut DBPooledConnection) -> Result<FoodItems, Error> {
    use crate::schema::food::dsl::*;

    let _food_items = match food
//        .order(created_at.desc())
        .limit(max_items)
        .load::<FoodDB>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };

    Ok(FoodItems {
        results: _food_items
            .into_iter()
            .map(|t| t.to_food())
            .collect::<Vec<Food>>(),
    })
}



fn get_food_item_by_id(_id: i32, conn: &mut DBPooledConnection) -> Result<Food, Error> {
    use crate::schema::food::dsl::*;

    let res = food.filter(id.eq(_id))
        .load::<FoodDB>(conn);
    match res {
        Ok(food_items_db) => match food_items_db.first() {
            Some(food_item_db) => Ok(food_item_db.to_food()),
            _ => Err(Error::NotFound),
        },
        Err(err) => Err(err),
    }
}

// TODO case insensitive
fn find_food_items_by_name(max_items: i64, _name: &String, conn: &mut DBPooledConnection) -> Result<FoodItems, Error> {
    use crate::schema::food::dsl::*;
    let pattern = format!("%{}%", _name);

    let _food_items = match food
//        .order(created_at.desc())
        .filter(name.like(pattern))
        .limit(max_items)
        .load::<FoodDB>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };

    Ok(FoodItems {
        results: _food_items
            .into_iter()
            .map(|t| t.to_food())
            .collect::<Vec<Food>>(),
    })
}

//TODO test case already existing food name
fn create_food_item(food_item: Food, conn: &mut DBPooledConnection) -> Result<Food, Error> {
    use crate::schema::food::dsl::*;

    let food_db_for_insert = food_item.to_food_db_for_insert();
    let inserted_row = 
        diesel::insert_into(food)
            .values(&food_db_for_insert)
            .get_result::<FoodDB>(conn)?;

    Ok(inserted_row.to_food())
}


fn delete_food_item(_id: i32, conn: &mut DBPooledConnection) -> Result<(), Error> {
    use crate::schema::food::dsl::*;

    let res = diesel::delete(food.filter(id.eq(_id)))
        .execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// update name based on id (for now)
//todo gestion erreur. cas où id not found
fn update_food_item(_id: i32, food_item: Food, conn: &mut DBPooledConnection) -> Result<Food, Error> {
    use crate::schema::food::dsl::*;

    let updated_row = 
        diesel::update(food)
        .filter(id.eq(_id))
        .set(name.eq(food_item.name))
        .get_result::<FoodDB>(conn)?;

    Ok(updated_row.to_food())
}



/// list all food items
#[get("api/food")]
pub async fn list(pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("called get without name");
    let food_items = 
        web::block(
            move || 
                list_food_items(
                    MAX_FOOD_ITEMS, 
                    &mut conn)
            ).await.unwrap().unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(food_items)
}


#[derive(Debug, Deserialize)]
pub struct FoodSearch {
   name: String
}

/// search food items by name
#[get("api/food/search/")]
pub async fn search(query_params: web::Query<FoodSearch>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    println!("name searched={}", query_params.name);
    let food_items = 
        web::block(
            move || 
            find_food_items_by_name(
                    MAX_FOOD_ITEMS,
                    &query_params.name, 
                    &mut conn)
            ).await.unwrap().unwrap();

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(food_items)
}

/// create a food item `/food`
#[post("api/food")]
pub async fn create(food_req: Json<FoodRequest>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);

    // TODO handle unwraps errors
    let food_item = 
        web::block(
            move || 
                create_food_item(
                    food_req.to_food().unwrap(), 
                    &mut conn)
            ).await.unwrap();

    match food_item {
        Ok(food_item) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(food_item),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
// TODO si not found ça panic
/// get a food item by its id `/food/{id}`
#[get("api/food/{id}")]
pub async fn get(path: Path<(i32,)>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    // TODO gestion d'erreur sur les unwrap
    let food_item =
        web::block(
            move || 
                get_food_item_by_id(
                    path.0, 
                    &mut conn)
            ).await.unwrap();

    match food_item {
        Ok(food_item) => {
            HttpResponse::Ok()
                .content_type(APPLICATION_JSON)
                .json(food_item)
        }
        _ => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a food item by its id `/food/{id}`
#[delete("api/food/{id}")]
pub async fn delete(path: Path<(i32,)>, pool: Data<DBPool>) -> HttpResponse {
    // in any case return status 204
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    let _ = web::block(
        move || 
            delete_food_item(
                path.0, &mut conn
            )
        ).await.unwrap();

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}


/// update a food item by its id `/food/{id}`
#[put("api/food/{id}")]
pub async fn update(path: Path<(i32,)>, food_req: Json<FoodRequest>, pool: Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect(CONNECTION_POOL_ERROR);
    // TODO handle unwraps errors
    let food_item = 
        web::block(
            move || 
                update_food_item(
                    path.0,
                    food_req.to_food().unwrap(),
                    &mut conn)
            ).await.unwrap();

    match food_item {
        Ok(food_item) => HttpResponse::Created()
            .content_type(APPLICATION_JSON)
            .json(food_item),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
