diesel::table! {
    routines (id) {
        id -> Int4,
        title -> Text,
        create_date -> Timestamptz,
        update_date -> Nullable<Timestamptz>,
        delete_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    routine_parts(id){
        id -> Int4,
        description -> Text,
        start_hour -> Timestamptz,
        end_hour -> Timestamptz,
        routine_id -> Int4,
    }
}

diesel::joinable!(routine_parts -> routines (routine_id));

diesel::allow_tables_to_appear_in_same_query!(routines, routine_parts);

diesel::table! {
    categories(id){
        id -> Int4,
        name ->  Text
    }
}
