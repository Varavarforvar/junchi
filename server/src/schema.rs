table! {
    posts (id) {
        id -> Unsigned<Bigint>,
        title -> Text,
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
