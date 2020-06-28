table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    t (id) {
        id -> Integer,
        c -> Nullable<Integer>,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    t,
);
