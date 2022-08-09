table! {
    usrs (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        active -> Bool,
    }
}
