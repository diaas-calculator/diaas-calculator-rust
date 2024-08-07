use diesel::result::Error;
use diesel::prelude::*;
//use log::warn;
use log::warn;

use diesel::{ExpressionMethods, RunQueryDsl, PgTextExpressionMethods, QueryDsl};
use crate::DBPooledConnection;
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::food::models;
use diesel::dsl::not;


pub fn get_food_item_by_id(_id: i32, conn: &mut DBPooledConnection) -> Result<Option<models::Food>, DbError> {
    use crate::schema::food::dsl::*;

    let food_item = food
        .filter(id.eq(_id))
        .first::<models::Food>(conn)
        .optional()?;

    Ok(food_item)
}

pub fn find_food_items(max_items: i64, param_name: &Option<String>, param_food_type: &Option<String>, param_aa_profile: &Option<String>, param_sort: &Option<String>, param_show_hidden: Option<bool>, conn: &mut DBPooledConnection) -> Result<Vec<models::Food>, Error>  {
    use crate::schema::food::dsl::*;

    // using into_boxed for conditional filtering
    let mut query = food.into_boxed();

    if let Some(some_name) = param_name {
        let pattern = format!("%{}%", some_name);
        query = query
            .filter(name.ilike(pattern));
    }

    if let Some(some_food_type) = param_food_type {
        query = query
            .filter(food_type.ilike(some_food_type));
    }

    if let Some(some_aa_profile) = param_aa_profile {
        let aa_profile_str = some_aa_profile.as_str();
        match aa_profile_str{
            "high-quality" => {
                query = query
                    .filter(histidine_score.ge(75.0))
                    .filter(isoleucine_score.ge(75.0))
                    .filter(leucine_score.ge(75.0))
                    .filter(lysine_score.ge(75.0))
                    .filter(saa_score.ge(75.0))
                    .filter(aaa_score.ge(75.0))
                    .filter(threonine_score.ge(75.0))
                    .filter(tryptophane_score.ge(75.0))
                    .filter(valine_score.ge(75.0))
            }
            "his+" => {
                query = query
                    .filter(histidine_score.gt(100.0))
            }
            "ile+" => {
                query = query
                    .filter(isoleucine_score.gt(100.0))
            }
            "leu+" => {
                query = query
                    .filter(leucine_score.gt(100.0))
            }
            "lys+" => {
                query = query
                    .filter(lysine_score.gt(100.0))
            }
            "saa+" => {
                query = query
                    .filter(saa_score.gt(100.0))
            }
            "aaa+" => {
                query = query
                    .filter(aaa_score.gt(100.0))
            }
            "thr+" => {
                query = query
                    .filter(threonine_score.gt(100.0))
            }
            "trp+" => {
                query = query
                    .filter(tryptophane_score.gt(100.0))
            }
            "val+" => {
                query = query
                    .filter(valine_score.gt(100.0))
            }
            _ => {}
        }
    }

    if let Some(some_show_hidden) = param_show_hidden {
        if !some_show_hidden{
            query = query.filter(not(hidden));
        }
    }
    else{
        // by default don't show hidden food items
        query = query.filter(not(hidden));
    }

    if let Some(some_sort) = param_sort {
        let param_sort_str: &str = some_sort;
        query = match param_sort_str{
            "name" => query.order(name),
            "protein_content" => query.order(protein_content.desc()),
            "histidine_score" => query.order(histidine_score.desc()),
            "isoleucine_score" => query.order(isoleucine_score.desc()),
            "leucine_score" => query.order(leucine_score.desc()),
            "lysine_score" => query.order(lysine_score.desc()),
            "saa_score" => query.order(saa_score.desc()),
            "aaa_score" => query.order(aaa_score.desc()),
            "threonine_score" => query.order(threonine_score.desc()),
            "tryptophane_score" => query.order(tryptophane_score.desc()),
            "valine_score" => query.order(valine_score.desc()),
            _ => {
                    warn!("find_food_items: We don't know this sort parameter: {}.", some_sort);
                    query.order(name)
                }
        };
    }
    else{
        query = query.order(name);
    }

    query = query.limit(max_items);

    let _food_items = query.select(models::Food::as_select()).load::<models::Food>(conn)?;
    Ok(_food_items)
    
}


pub fn find_food_items_i18n(max_items: i64, param_name: &Option<String>, param_food_type: &Option<String>, param_aa_profile: &Option<String>, param_lang: &String, param_sort: &Option<String>, param_show_hidden: Option<bool>, conn: &mut DBPooledConnection) -> Result<Vec<(models::Food,models::FoodI18n)>, Error>  {
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

    if let Some(some_food_type) = param_food_type {
        let pattern = format!("%{}%", some_food_type);
        query = query
            .filter(food_type.ilike(pattern));
    }

    if let Some(some_aa_profile) = param_aa_profile {
        let aa_profile_str = some_aa_profile.as_str();
        match aa_profile_str{
            "high-quality" => {
                query = query
                    .filter(histidine_score.ge(75.0))
                    .filter(isoleucine_score.ge(75.0))
                    .filter(leucine_score.ge(75.0))
                    .filter(lysine_score.ge(75.0))
                    .filter(saa_score.ge(75.0))
                    .filter(aaa_score.ge(75.0))
                    .filter(threonine_score.ge(75.0))
                    .filter(tryptophane_score.ge(75.0))
                    .filter(valine_score.ge(75.0))
            }
            "his+" => {
                query = query
                    .filter(histidine_score.gt(100.0))
            }
            "ile+" => {
                query = query
                    .filter(isoleucine_score.gt(100.0))
            }
            "leu+" => {
                query = query
                    .filter(leucine_score.gt(100.0))
            }
            "lys+" => {
                query = query
                    .filter(lysine_score.gt(100.0))
            }
            "saa+" => {
                query = query
                    .filter(saa_score.gt(100.0))
            }
            "aaa+" => {
                query = query
                    .filter(aaa_score.gt(100.0))
            }
            "thr+" => {
                query = query
                    .filter(threonine_score.gt(100.0))
            }
            "trp+" => {
                query = query
                    .filter(tryptophane_score.gt(100.0))
            }
            "val+" => {
                query = query
                    .filter(valine_score.gt(100.0))
            }
            _ => {}
        }
    }

    if let Some(some_show_hidden) = param_show_hidden {
        if !some_show_hidden{
            query = query.filter(not(hidden));
        }
    }
    else{
        // by default don't show hidden food items
        query = query.filter(not(hidden));
    }

    if let Some(some_sort) = param_sort {
        let param_sort_str: &str = some_sort;
        query = match param_sort_str{
            "name" => query.order(name_translation),
            "protein_content" => query.order(protein_content.desc()),
            "histidine_score" => query.order(histidine_score.desc()),
            "isoleucine_score" => query.order(isoleucine_score.desc()),
            "leucine_score" => query.order(leucine_score.desc()),
            "lysine_score" => query.order(lysine_score.desc()),
            "saa_score" => query.order(saa_score.desc()),
            "aaa_score" => query.order(aaa_score.desc()),
            "threonine_score" => query.order(threonine_score.desc()),
            "tryptophane_score" => query.order(tryptophane_score.desc()),
            "valine_score" => query.order(valine_score.desc()),
            _ => {
                    warn!("find_food_items_i18n: We don't know this sort parameter: {}.", some_sort);
                    query.order(name_translation)
                }
        };
    }
    else{
        query = query.order(name_translation);
    }


    query = query.limit(max_items);

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

