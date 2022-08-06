table! {
    usrs (id) {
        id -> Int4,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        active -> Bool,
    }
}
