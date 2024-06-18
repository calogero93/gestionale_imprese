diesel::table! {

    employees (id) {
        id -> Int4,
        nome -> Varchar,
        cognome -> Varchar,
        ruolo -> Varchar,
    }
}

macro_rules! define_schema {
    ($schema_name:ident) => {
        pub mod $schema_name {
            table! {
                use diesel::sql_types::*;
                use crate::models::*;

                $schema_name.employees (id) {
                    id -> Int4,
                    nome -> Varchar,
                    cognome -> Varchar,
                    ruolo -> Varchar,
                }
            }
        }
    };
}