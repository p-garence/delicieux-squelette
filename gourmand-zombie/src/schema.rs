// @generated automatically by Diesel CLI.

diesel::table! {
    dessins (id) {
        id -> Int4,
        x -> Int4,
        y -> Int4,
        done -> Bool,
        room_id -> Int4,
        content -> Nullable<Bytea>,
        last_request -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        name -> Text,
        rules -> Nullable<Text>,
        is_public -> Bool,
        password_protected -> Bool,
        hashed_password -> Nullable<Text>,
        colors -> Array<Nullable<Text>>,
        resolution -> Int4,
        number_of_dessin -> Nullable<Int4>,
        created_at -> Timestamp,
        hashed_admin -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(dessins, rooms,);
