use std::fmt::format;
use std::hash::DefaultHasher;
use std::hash::Hasher;

use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::PgConnection;
use regex::Regex;
use crate::models;
use crate::schema;
use std::hash::Hash;

pub async fn set_search_path(conn: &mut PgConnection, schema_name: &str) -> Result<(), String> {
    diesel::sql_query(format!("SET search_path TO {}", schema_name))
        .execute(conn).unwrap();
    Ok(())
}

pub fn get_qualifiche(conn: &mut PgConnection, schema_name: &str) -> Result<Vec<models::Qualifiche>, String> {
    use schema::qualifiches::dsl::*;
    let result = qualifiches.load::<models::Qualifiche>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn get_imprese(conn: &mut PgConnection) -> Result<Vec<models::Imprese>, String> {
    use schema::impreses::dsl::*;
    let result = impreses.load::<models::Imprese>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn get_imprese_associate_utenti(conn: &mut PgConnection, id_utente: i32) -> Result<Vec<models::ImpreseAssociateUtenti>, String> {
    use schema::imprese_associate_utentis::dsl::*;
    let result = imprese_associate_utentis.filter(utente_id.eq(id_utente)).load::<models::ImpreseAssociateUtenti>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn get_utente(conn: &mut PgConnection, id_utente: i32) -> Result<models::Utenti, String> {
    use schema::utentis::dsl::*;
    let result = utentis.filter(id.eq(id_utente)).first::<models::Utenti>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn get_mezzo(conn: &mut PgConnection, id_mezzo: i32) -> Result<models::Mezzi, String> {
    use schema::mezzis::dsl::*;
    let result = mezzis.filter(id.eq(id_mezzo)).first::<models::Mezzi>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn get_autovettura(conn: &mut PgConnection, id_autovettura: i32) -> Result<models::Autovetture, String> {
    use schema::autovettures::dsl::*;
    let result = autovettures.filter(id.eq(id_autovettura)).first::<models::Autovetture>(conn).map_err(|e| format!("{}", e));
    Ok(result?)
}

pub fn hashing<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}


pub fn parse_week_start_date(week_str: &str) -> Result<NaiveDate, String> {
    let re = Regex::new(r"(\d{4}) - Week \d+ - (\d{1,2})/(\d{1,2})").unwrap();
    
    if let Some(captures) = re.captures(week_str) {
        let year: i32 = captures[1].parse().unwrap();
        let day: u32 = captures[2].parse().unwrap();
        let month: u32 = captures[3].parse().unwrap();

        NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            "Invalid date format in week string".to_string()
        })
    } else {
        Err("Invalid week string format".to_string())
    }
}