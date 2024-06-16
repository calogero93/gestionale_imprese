use super::schema::users;
use super::schema::employees;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}


#[derive(Queryable, Serialize, AsChangeset)]
pub struct Employee {
    pub id: i32,
    pub nome: String,
    pub cognome: String,
    pub ruolo: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "employees"]
pub struct NewEmployee {
    pub nome: String,
    pub cognome: String,
    pub ruolo: String,
}
