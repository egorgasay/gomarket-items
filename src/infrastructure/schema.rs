// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int8,
        name -> Varchar,
        description -> Varchar,
        price -> Float8,
    }
}

diesel::table! {
    items_sizes (id) {
        id -> Int8,
        item_id -> Int8,
        size_id -> Int4,
        quantity -> Int4,
    }
}

diesel::table! {
    sizes (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(items_sizes -> items (item_id));
diesel::joinable!(items_sizes -> sizes (size_id));

diesel::allow_tables_to_appear_in_same_query!(
    items,
    items_sizes,
    sizes,
);
