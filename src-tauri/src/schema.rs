// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Nullable<Integer>,
        name -> Text,
        hours -> Integer,
        overtime -> Integer,
    }
}

diesel::table! {
    settings (id) {
        id -> Nullable<Integer>,
        skey -> Text,
        value -> Text,
    }
}

diesel::table! {
    workdays (id) {
        id -> Nullable<Integer>,
        start -> Integer,
        end -> Integer,
        employee_id -> Integer,
        holiday -> Integer,
    }
}

diesel::joinable!(workdays -> employees (employee_id));

diesel::allow_tables_to_appear_in_same_query!(
    employees,
    settings,
    workdays,
);
