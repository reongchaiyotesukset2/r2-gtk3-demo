// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    providers (id) {
        id -> Integer,
        name -> Text,
        website -> Nullable<Text>,
        help_url -> Nullable<Text>,
        image_uri -> Nullable<Text>,
        digits -> Nullable<Integer>,
        period -> Nullable<Integer>,
        default_counter -> Nullable<Integer>,
        algorithm -> Nullable<Text>,
        method -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(posts, providers,);
