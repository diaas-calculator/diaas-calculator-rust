use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::super::schema::{mix,mix_food};
use super::super::food::models::Food;

/* 
* Tables
*/
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = mix)]
pub struct Mix {
    pub id: i32,
    pub visibility: i16,
    pub name: String,
    pub description: Option<String>
}


impl Mix {
    pub fn new(
        visibility: i16,
        name: String,
        description: Option<String>
            ) -> Self {
        Self {
            // will be set by the database
            id: 0,
            visibility,
            name,
            description
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = mix_food)]
pub struct MixFood {
    pub mix_id: i32,
    pub food_id: i32,
    pub food_weight: f32
}

/* 
* Objects resulted from sql joins
*/
#[derive(Deserialize, Serialize, Queryable)]
pub struct MixFoodJoin{
    pub id: i32,
    pub food_weight: f32,
    pub food: Food,
}

#[derive(Deserialize, Serialize, Queryable)]
pub struct MixFoodJoinI18n{
    pub id: i32,
    pub food_weight: f32,
    pub name_translation: String,
    pub food: Food,
}