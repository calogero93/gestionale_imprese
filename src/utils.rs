use diesel::prelude::*;
use diesel::PgConnection;

pub fn set_search_path(conn: &mut PgConnection, schema_name: &str) -> Result<(), String> {
    diesel::sql_query(format!("SET search_path TO {}", schema_name))
        .execute(conn).unwrap();
    Ok(())
}