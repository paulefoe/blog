table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        body -> Text,
        published -> Bool,
        created_at -> Nullable<Timestamp>,
        views_count -> Nullable<Integer>,
        description -> Text,
    }
}
