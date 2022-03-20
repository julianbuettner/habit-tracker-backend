table! {
    habitmetric (id) {
        id -> Int4,
        habitname_id -> Int4,
        datetime -> Timestamp,
        units -> Int4,
    }
}

table! {
    habitname (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(habitmetric -> habitname (habitname_id));

allow_tables_to_appear_in_same_query!(habitmetric, habitname,);
