use diesel::prelude::*;
use super::super::schema::{food,food_i18n};
use serde::{Deserialize, Serialize};

//TODO handle reference_details and comment null
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = food)]
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
    pub comment: String,
    pub hidden: bool,
    // the greenhouse_gas in kg of eqCo2 per kg of food
    pub greenhouse_gas: f32,
    // the food item used in the greenhouse gas database
    pub greenhouse_gas_ref: Option<String>,
    pub greenhouse_gas_link: Option<String>,
    pub greenhouse_gas_comment: Option<String>
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
            comment: String,
            hidden: bool,
            greenhouse_gas: f32,
            greenhouse_gas_ref: Option<String>,
            greenhouse_gas_link: Option<String>,
            greenhouse_gas_comment: Option<String>
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
            comment,
            hidden,
            greenhouse_gas,
            greenhouse_gas_ref,
            greenhouse_gas_link,
            greenhouse_gas_comment
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
            comment: self.comment.clone(),
            hidden: self.hidden,
            greenhouse_gas: self.greenhouse_gas,
            greenhouse_gas_ref: self.greenhouse_gas_ref.clone(),
            greenhouse_gas_link: self.greenhouse_gas_link.clone(),
            greenhouse_gas_comment: self.greenhouse_gas_comment.clone()
        }
    }
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
    pub comment: String,
    pub hidden: bool,
    pub greenhouse_gas: f32,
    pub greenhouse_gas_ref: Option<String>,
    pub greenhouse_gas_link: Option<String>,
    pub greenhouse_gas_comment: Option<String>
}

// For creating (and updating) a food (without an id)
#[derive(Debug, Deserialize, Serialize)]
pub struct NewFood {
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
    pub comment: Option<String>,
    pub hidden: Option<bool>,
    pub greenhouse_gas: Option<f32>,
    pub greenhouse_gas_ref: Option<String>,
    pub greenhouse_gas_link: Option<String>,
    pub greenhouse_gas_comment: Option<String>
}

impl NewFood {
    pub fn to_food(&self) -> Option<Food> {
        match (&self.name, self.protein_content, &self.food_type, &self.score_type, &self.protein_content_cooked_state, &self.diaas_cooked_state, self.histidine_score, self.isoleucine_score, self.leucine_score, self.lysine_score, self.saa_score, self.aaa_score, self.threonine_score, self.tryptophane_score, self.valine_score, &self.reference_link, &self.reference_details, &self.comment, self.hidden, self.greenhouse_gas, &self.greenhouse_gas_ref, &self.greenhouse_gas_link, &self.greenhouse_gas_comment) {
            (Some(name), Some(protein_content), Some(food_type), Some(score_type), Some(protein_content_cooked_state), Some(diaas_cooked_state), Some(histidine_score), Some(isoleucine_score), Some(leucine_score), Some(lysine_score), Some(saa_score), Some(aaa_score), Some(threonine_score), Some(tryptophane_score), Some(valine_score), Some(reference_link), Some(reference_details), Some(comment), Some(hidden), Some(greenhouse_gas), greenhouse_gas_ref, greenhouse_gas_link, greenhouse_gas_comment) =>  
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
                            comment.to_string(),
                            hidden,
                            greenhouse_gas,
                            greenhouse_gas_ref.clone(),
                            greenhouse_gas_link.clone(),
                            greenhouse_gas_comment.clone()
                        )
                    ),
            _ => None
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = food_i18n)]
pub struct FoodI18n {
    pub lang: String,
    pub food_id: i32,
    pub name_translation: String
}
