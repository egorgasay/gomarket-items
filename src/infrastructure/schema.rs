// @generated automatically by Diesel CLI.

diesel::table! {
    mechanics (match_word) {
        match_word -> Varchar,
        reward -> Float8,
        reward_type -> Varchar,
    }
}
