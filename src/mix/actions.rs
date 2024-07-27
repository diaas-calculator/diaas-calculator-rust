use diesel::prelude::*;

use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};
use crate::constants::MIX_VISIBILITY_EXAMPLE;
use crate::DBPooledConnection;
type DbError = Box<dyn std::error::Error + Send + Sync>;
use crate::mix::models;

pub fn get_mix_by_id(_id: i32, conn: &mut DBPooledConnection) -> Result<Option<models::Mix>, DbError> {
    use crate::schema::mix::dsl::*;

    let my_mix = mix
        .filter(id.eq(_id))
        .first::<models::Mix>(conn)
        .optional()?;

    Ok(my_mix)
}

// For now only list example mixes
pub fn list_mixes(max_items: i64, conn: &mut DBPooledConnection) -> Result<Vec<models::Mix>, DbError> {
    use crate::schema::mix::dsl::*;

    let mixes = mix
        .filter(visibility.eq(MIX_VISIBILITY_EXAMPLE))
        .limit(max_items)
        .load::<models::Mix>(conn)?;

    Ok(mixes)
}

pub fn get_mix_by_id_with_food(_id: i32, conn: &mut DBPooledConnection) -> Result<Vec<models::MixFoodJoin>, DbError> {
    use crate::schema::mix::dsl::*;
    use crate::schema::mix_food::dsl::*;
    use crate::schema::food::dsl::*;

    let join = mix_food.inner_join(food).inner_join(mix)
        .select((crate::schema::mix::dsl::id, food_weight, food::all_columns()));

    let my_mix = join
        .filter(crate::schema::mix::dsl::id.eq(_id))
        .load(conn)?;

    Ok(my_mix)
}

pub fn get_mix_by_id_with_food_i18n(_id: i32, conn: &mut DBPooledConnection) -> Result<Vec<models::MixFoodJoinI18n>, DbError> {
    use crate::schema::mix::dsl::*;
    use crate::schema::mix_food::dsl::*;
    use crate::schema::food::dsl::*;
    use crate::schema::food_i18n::dsl::*;

    let join = mix_food.inner_join(food.inner_join(food_i18n)).inner_join(mix)
        .select((crate::schema::mix::dsl::id, food_weight, name_translation, food::all_columns()));

    let my_mix = join
        .filter(crate::schema::mix::dsl::id.eq(_id))
        .load(conn)?;

    Ok(my_mix)
}

