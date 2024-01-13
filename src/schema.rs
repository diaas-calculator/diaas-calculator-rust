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
