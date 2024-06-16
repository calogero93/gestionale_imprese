use diesel::prelude::*;
use diesel::PgConnection;

pub fn run(schema_name: &str, conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let create_table_query = format!(
        r#"
        CREATE TABLE {schema}.example_table (
            id SERIAL PRIMARY KEY,
            data TEXT NOT NULL
        );
        "#,
        schema = schema_name
    );

    diesel::sql_query(create_table_query).execute(conn)?;

    Ok(())
}