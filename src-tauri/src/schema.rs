// @generated automatically by Diesel CLI.

diesel::table! {
    author (id) {
        id -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
        url -> Text,
    }
}

diesel::table! {
    category (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    cuisine (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    ingredient (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    rating (id) {
        id -> Nullable<Integer>,
        rating -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    receipt (id) {
        id -> Nullable<Integer>,
        name -> Text,
        cookTime -> Integer,
        prepTime -> Integer,
        #[sql_name = "yield"]
        yield_ -> Integer,
        authorId -> Integer,
        ratingId -> Integer,
        categoryId -> Integer,
    }
}

diesel::table! {
    receipt_cuisine (id) {
        id -> Nullable<Integer>,
        receiptId -> Integer,
        cuisineId -> Integer,
    }
}

diesel::table! {
    receipt_ingredient (id) {
        id -> Nullable<Integer>,
        receiptId -> Integer,
        ingredientId -> Integer,
        unitId -> Integer,
        amount -> Text,
    }
}

diesel::table! {
    step (id) {
        id -> Nullable<Integer>,
        receiptId -> Integer,
        step -> Integer,
        description -> Text,
    }
}

diesel::table! {
    unit (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::joinable!(receipt -> author (authorId));
diesel::joinable!(receipt -> category (categoryId));
diesel::joinable!(receipt -> rating (ratingId));
diesel::joinable!(receipt_cuisine -> cuisine (cuisineId));
diesel::joinable!(receipt_cuisine -> receipt (receiptId));
diesel::joinable!(receipt_ingredient -> ingredient (ingredientId));
diesel::joinable!(receipt_ingredient -> receipt (receiptId));
diesel::joinable!(receipt_ingredient -> unit (unitId));
diesel::joinable!(step -> receipt (receiptId));

diesel::allow_tables_to_appear_in_same_query!(
    author,
    category,
    cuisine,
    ingredient,
    rating,
    receipt,
    receipt_cuisine,
    receipt_ingredient,
    step,
    unit,
);
