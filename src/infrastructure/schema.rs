// @generated automatically by Diesel CLI.
//
// diesel::table! {
//     service_contexts (id) {
//         id -> Int4,
//         maintenance -> Bool,
//     }
// }

diesel::table! {
    orders (id) {
        id -> Varchar,
        title -> Varchar,
        description -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    goods (id) {
        id -> Varchar,
        price -> Double,
    }
}

diesel::table! {
    orders_goods (order_id, good_id) {
        order_id -> Varchar, // внешний ключ на таблицу orders
        good_id -> Varchar, // внешний ключ на таблицу goods
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    // service_contexts,
    orders,
    orders_goods,
    goods,
);
