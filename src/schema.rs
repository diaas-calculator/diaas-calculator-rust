// @generated automatically by Diesel CLI.

diesel::table! {
    food (id) {
        id -> Int4,
        name -> Varchar,
        protein_content -> Float4,
        food_type -> Varchar,
        score_type -> Varchar,
        protein_content_cooked_state -> Varchar,
        diaas_cooked_state -> Varchar,
        histidine_score -> Float4,
        isoleucine_score -> Float4,
        leucine_score -> Float4,
        lysine_score -> Float4,
        saa_score -> Float4,
        aaa_score -> Float4,
        tryptophane_score -> Float4,
        threonine_score -> Float4,
        valine_score -> Float4,
        reference_link -> Varchar,
        reference_details -> Varchar,
        comment -> Varchar,
    }
}

diesel::table! {
    food_i18n (lang, food_id, name_translation) {
        lang -> Varchar,
        food_id -> Int4,
        name_translation -> Varchar,
    }
}

diesel::joinable!(food_i18n -> food (food_id));

diesel::allow_tables_to_appear_in_same_query!(
    food,
    food_i18n,
);
