use diesel::result::Error;
use diesel::prelude::*;
//use log::{trace,error};

use diesel::{ExpressionMethods, RunQueryDsl, PgTextExpressionMethods, QueryDsl};
use crate::DBPooledConnection;
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::food::models;

pub fn list_food_items(max_items: i64, conn: &mut DBPooledConnection) -> Result<models::FoodItems, Error> {
    use crate::schema::food::dsl::*;

    let _food_items = match food
        .order(name)
        .limit(max_items)
        .load::<models::Food>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };

    Ok(models::FoodItems {
        results: _food_items
            .into_iter()
            .collect::<Vec<models::Food>>(),
    })
}


pub fn list_food_items_i18n(max_items: i64, param_lang: &String, conn: &mut DBPooledConnection) -> Result<models::FoodItemsI18n, Error> {
    use crate::schema::food::dsl::*;
    use crate::schema::food_i18n::dsl::*;
    let _food_items: Vec<models::Food>;

    let _food_items_i18n = match food
    // TODO outer join to handle missing translation
    .inner_join(food_i18n)
    .filter(lang.eq(param_lang))
    .order(name)
    .limit(max_items)
    .load::<(models::Food,models::FoodI18n)>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };
    

    Ok(models::FoodItemsI18n {
        results: _food_items_i18n
            .into_iter()
            .collect::<Vec<(models::Food,models::FoodI18n)>>(),
    })
}


pub fn get_food_item_by_id(_id: i32, conn: &mut DBPooledConnection) -> Result<Option<models::Food>, DbError> {
    use crate::schema::food::dsl::*;

    let food_item = food
        .filter(id.eq(_id))
        .first::<models::Food>(conn)
        .optional()?;

    Ok(food_item)
}

pub fn find_food_items_by_name(max_items: i64, _name: &String, conn: &mut DBPooledConnection) -> Result<models::FoodItems, Error> {
    use crate::schema::food::dsl::*;
    let pattern = format!("%{}%", _name);

    let _food_items = match food
        .order(name)
        .filter(name.ilike(pattern))
        .limit(max_items)
        .load::<models::Food>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };

    Ok(models::FoodItems {
        results: _food_items
            .into_iter()
            .collect::<Vec<models::Food>>(),
    })
}


pub fn find_food_items_by_name_i18n(max_items: i64, _name: &String, param_lang: &String, conn: &mut DBPooledConnection) -> Result<models::FoodItemsI18n, Error> {
    use crate::schema::food::dsl::*;
    use crate::schema::food_i18n::dsl::*;
    let pattern = format!("%{}%", _name);

    let _food_items_i18n = match food
        .inner_join(food_i18n)
        .filter(lang.eq(param_lang))
        .filter(name_translation.ilike(pattern))
        .order(name_translation)
        .limit(max_items)
        .load::<(models::Food,models::FoodI18n)>(conn)
    {
        Ok(fditems) => fditems,
        Err(_) => vec![],
    };

    Ok(models::FoodItemsI18n {
        results: _food_items_i18n
            .into_iter()
            .collect::<Vec<(models::Food,models::FoodI18n)>>(),
    })
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

