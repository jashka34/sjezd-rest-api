table! {
    usrs (id) {
        id -> Nullable<Integer>,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        active -> Bool,
        hash_psw -> Text,
    }
}
