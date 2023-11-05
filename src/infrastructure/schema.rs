// @generated automatically by Diesel CLI.

diesel::table! {
    goods (id) {
        id -> Varchar,
        price -> Float8,
    }
}

diesel::table! {
    orders (id) {
        id -> Varchar,
    }
}

diesel::table! {
    orders_goods (order_id) {
        order_id -> Varchar,
        good_id -> Nullable<Varchar>,
    }
}

diesel::joinable!(orders_goods -> goods (good_id));
diesel::joinable!(orders_goods -> orders (order_id));

diesel::allow_tables_to_appear_in_same_query!(
    goods,
    orders,
    orders_goods,
);
