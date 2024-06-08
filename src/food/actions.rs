use diesel::result::Error;
use diesel::prelude::*;
//use log::{trace,error};

use diesel::{ExpressionMethods, RunQueryDsl, PgTextExpressionMethods, QueryDsl};
use crate::DBPooledConnection;
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::food::models;


pub fn get_food_item_by_id(_id: i32, conn: &mut DBPooledConnection) -> Result<Option<models::Food>, DbError> {
    use crate::schema::food::dsl::*;

    let food_item = food
        .filter(id.eq(_id))
        .first::<models::Food>(conn)
        .optional()?;

    Ok(food_item)
}

pub fn find_food_items(max_items: i64, param_name: &Option<String>, conn: &mut DBPooledConnection) -> Result<Vec<models::Food>, Error>  {
    use crate::schema::food::dsl::*;

    // using into_boxed for conditional filtering
    let mut query = food.into_boxed();

    if let Some(some_name) = param_name {
        let pattern = format!("%{}%", some_name);
        query = query
            .filter(name.ilike(pattern));
    }

    query = query
        .order(name)
        .limit(max_items);

    let _food_items = query.select(models::Food::as_select()).load::<models::Food>(conn)?;
    Ok(_food_items)
    
}


pub fn find_food_items_i18n(max_items: i64, param_name: &Option<String>, param_lang: &String, conn: &mut DBPooledConnection) -> Result<Vec<(models::Food,models::FoodI18n)>, Error>  {
    use crate::schema::food::dsl::*;
    use crate::schema::food_i18n::dsl::*;

    let join = food.inner_join(food_i18n);

    // using into_boxed for conditional filtering
    let mut query = join.into_boxed();

    query = query
        .filter(lang.eq(param_lang));

    if let Some(some_name) = param_name {
        let pattern = format!("%{}%", some_name);
        query = query
            .filter(name_translation.ilike(pattern));
    }

    query = query
        .order(name_translation)
        .limit(max_items);

    let _food_items = query.select((models::Food::as_select(),models::FoodI18n::as_select())).load::<(models::Food,models::FoodI18n)>(conn)?;
    Ok(_food_items)
    
}



pub fn create_food_item(food_item: models::Food, conn: &mut DBPooledConnection) -> Result<models::Food, Error> {
    use crate::schema::food::dsl::*;

    let food_db_for_insert = food_item.to_food_db_for_insert();
    let inserted_row = 
        diesel::insert_into(food)
            .values(&food_db_for_insert)
            .get_result::<models::Food>(conn)?;

    Ok(inserted_row)
}


pub fn delete_food_item(_id: i32, conn: &mut DBPooledConnection) -> Result<(), Error> {
    use crate::schema::food::dsl::*;

    let res = diesel::delete(food.filter(id.eq(_id)))
        .execute(conn);
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// update name based on id (for now)
pub fn update_food_item(_id: i32, food_item: models::Food, conn: &mut DBPooledConnection) -> Result<models::Food, Error> {
    use crate::schema::food::dsl::*;

    let updated_row = 
        diesel::update(food)
        .filter(id.eq(_id))
        .set(name.eq(food_item.name))
        .get_result::<models::Food>(conn)?;

    Ok(updated_row)
}

