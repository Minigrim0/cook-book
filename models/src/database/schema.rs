// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
        url -> Text,
    }
}

diesel::table! {
    category (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    cuisine (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    image_blobs (id) {
        id -> Integer,
        image_data -> Binary,
        created_at -> Timestamp,
        hash -> Text,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    job (id) {
        id -> Integer,
        status -> Text,
        progress -> Float,
        details -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    job_log (id) {
        id -> Integer,
        job_id -> Integer,
        log_entry -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rating (id) {
        id -> Integer,
        score -> Float,
        amount -> Integer,
    }
}

diesel::table! {
    recipe (id) {
        id -> Integer,
        name -> Text,
        cook_time -> Integer,
        prep_time -> Integer,
        #[sql_name = "yield"]
        yield_ -> Integer,
        author_id -> Integer,
        rating_id -> Integer,
        category_id -> Integer,
    }
}

diesel::table! {
    recipe_cuisine (id) {
        id -> Integer,
        recipe_id -> Integer,
        cuisine_id -> Integer,
    }
}

diesel::table! {
    recipe_images (recipe_id, image_id) {
        recipe_id -> Integer,
        image_id -> Integer,
    }
}

diesel::table! {
    recipe_ingredient (id) {
        id -> Integer,
        recipe_id -> Integer,
        ingredient_id -> Integer,
        unit_id -> Integer,
        amount -> Float,
        details -> Nullable<Text>,
        full -> Nullable<Text>,
    }
}

diesel::table! {
    step (id) {
        id -> Integer,
        recipe_id -> Integer,
        number -> Integer,
        description -> Text,
    }
}

diesel::table! {
    unit (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(job_log -> job (job_id));
diesel::joinable!(recipe -> author (author_id));
diesel::joinable!(recipe -> category (category_id));
diesel::joinable!(recipe -> rating (rating_id));
diesel::joinable!(recipe_cuisine -> cuisine (cuisine_id));
diesel::joinable!(recipe_cuisine -> recipe (recipe_id));
diesel::joinable!(recipe_images -> image_blobs (image_id));
diesel::joinable!(recipe_ingredient -> ingredient (ingredient_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(recipe_ingredient -> unit (unit_id));
diesel::joinable!(step -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    category,
    cuisine,
    image_blobs,
    ingredient,
    job,
    job_log,
    rating,
    recipe,
    recipe_cuisine,
    recipe_images,
    recipe_ingredient,
    step,
    unit,
);
