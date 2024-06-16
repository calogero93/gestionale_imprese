// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
    
}

diesel::table! {

    employees (id) {
        id -> Int4,
        nome -> Varchar,
        cognome -> Varchar,
        ruolo -> Varchar,
    }
}
