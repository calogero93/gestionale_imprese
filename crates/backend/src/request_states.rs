use chrono::NaiveDate;
use diesel::query_builder::AsChangeset;
use serde::Deserialize;
use crate::schema::{self, *};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AddUserRequest {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub role: String,
    pub state: bool,
    pub name: String,
    pub surname: String,
    pub company_id: i32,
}

#[derive(Deserialize)]
pub struct Assegnamenti { 

    pub dipendente_id: i32,
    pub autovettura_id: i32

}

#[derive(Deserialize)]
pub struct AddSettimanaleRequest {
    pub assegnazione: Vec<Assegnamenti>,
    pub data_settimanale: String,
    //pub luogo_di_nascita: String,
    //pub data_di_nascita: NaiveDate,
    //pub tipo_proprieta: i32,
    //pub proprieta: String,
    pub impresa_id: i32,
    pub opera_id: i32,
    //pub mezzo_id: Option<i32>,
    //pub autovettura_id: i32,
    //pub matricola: Option<String>,
    //pub targa: String,
}


#[derive(Deserialize)]
pub struct AddAutovettureRequest {
    pub descrizione: Option<String>,
    pub modello: String,
    pub targa: String,
    pub tipo_proprieta: i32,
    pub proprieta: String,
    pub impresa_id: i32,
    pub data_dimissioni: NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Deserialize)]
pub struct AddDipendentiRequest {
    pub nome: String,
    pub cognome: String,
    pub matricola: Option<String>,
    pub data_di_nascita: chrono::NaiveDate,
    pub luogo_di_nascita: String,
    pub codice_fiscale: String,
    pub impresa_id: i32,
    pub qualifica: i32,
    pub mansione: i32,
    pub data_dimissioni: Option<chrono::NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}

#[derive(Deserialize)]
pub struct AddImpreseAssociateUtentisRequest {
    pub utente_id: i32,
    pub impresa_id: i32,
}

#[derive(Deserialize)]
pub struct AddImpreseCollegateRequest {
    pub impresa_id: i32,
    pub ruolo_impresa: String,
    pub imprese_collegata_id: i32
}

#[derive(Deserialize)]
pub struct AddImpreseRequest {
    pub ragione_sociale: Option<String>,
    pub indirizzo: String,
    pub partita_iva: String,
}

#[derive(Deserialize)]
pub struct AddMansioniRequest {
    pub descrizione: String,
}

#[derive(Deserialize)]
pub struct AddMezziRequest {
    pub descrizione: Option<String>,
    pub modello: String,
    pub tipo_proprieta: i32,
    pub proprieta: String,
    pub impresa_id: i32,
    pub data_dimissioni: chrono::NaiveDate,
    pub rfid1: String,
    pub rfid2: String,
}

#[derive(Deserialize)]
pub struct AddOpereRequest {
    pub descrizione: String,
}

#[derive(Deserialize)]
pub struct AddQualificheRequest {
    pub descrizione: String,
}

#[derive(Deserialize)]
pub struct AddTipiProprietaRequest {
    pub descrizione: String,
}

#[derive(Deserialize)]
pub struct UserRequest {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub impresa_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct AddEmployeeRequest {
    pub nome: String,
    pub cognome: String,
    pub ruolo: String,
    pub data_di_nascita: chrono::NaiveDate,
    pub luogo_di_nascita: String,
    pub codice_fiscale: String,
    pub impresa_id: i32,
    pub qualifica_id: i32,
    pub mansione_id: i32
}

#[derive(Deserialize)]
pub struct GetUserDataQuery {
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub ruolo: Option<String>,
}

#[derive(Deserialize)]
pub struct GetEmployeeDataQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct UpdateEmployeeRequest {
    pub id: i32,
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub ruolo: Option<String>,
}



/*#[derive(Deserialize, AsChangeset, Debug)]
#[diesel(table_name = employees)]
pub struct UpdateEmployeeFields {
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub ruolo: Option<String>,
}*/

#[derive(Deserialize)]
pub struct RemoveEmployeeQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct ChangePasswordRequest {
    pub id: i32,
    pub old_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct UpdateChangePassword {
    pub password: String,
    pub primo_login: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateQualificheRequest {
    pub id: i32,
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateMansioniRequest {
    pub id: i32,
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSettimanale {
    pub id: i32,
    pub data_settimanale: Option<String>,
    pub utente_id: Option<i32>,
    pub luogo_di_nascita: Option<String>,
    pub data_di_nascita: Option<NaiveDate>,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: Option<String>,
    pub impresa_id: Option<i32>,
    pub opera_id: Option<i32>,
    pub mezzo_id: Option<i32>,
    pub autovettura_id: Option<i32>,
    pub matricola: Option<String>,
    pub targa: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateOpereRequest {
    pub id: i32,
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTipiProprietaRequest {
    pub id: i32,
    pub descrizione: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateImpreseRequest {
    pub id: i32,
    pub ragione_sociale: Option<String>,
    pub indirizzo: Option<String>,
    pub partita_iva: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateImpreseCollegateRequest {
    pub id: i32,
    pub impresa_id: Option<i32>,
    pub ruolo_impresa: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUtentiRequest {
    pub id: i32,
    pub username: Option<String>,
    pub password: Option<String>,
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub impresa_id: Option<i32>,
    pub utente: Option<String>,
    pub autorizazzione: Option<bool>,
    pub primo_login: Option<bool>,
    pub super_utente: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateImpreseAssociateUtentisRequest {
    pub id: i32,
    pub utente_id: Option<i32>,
    pub impresa_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateDipendentiRequest {
    pub id: i32,
    pub nome: Option<String>,
    pub cognome: Option<String>,
    pub matricola: Option<String>,
    pub data_di_nascita: Option<chrono::NaiveDate>,
    pub luogo_di_nascita: Option<String>,
    pub codice_fiscale: Option<String>,
    pub impresa_id: Option<i32>,
    pub qualifica: Option<i32>,
    pub mansione: Option<i32>,
    pub data_dimissioni: Option<chrono::NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateMezziRequest {
    pub id: i32,
    pub descrizione: Option<String>,
    pub modello: Option<String>,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: Option<String>,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: Option<chrono::NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateAutovettureRequest {
    pub id: i32,
    pub descrizione: Option<String>,
    pub modello: Option<String>,
    pub targa: Option<String>,
    pub tipo_proprieta: Option<i32>,
    pub proprieta: Option<String>,
    pub impresa_id: Option<i32>,
    pub data_dimissioni: Option<chrono::NaiveDate>,
    pub rfid1: Option<String>,
    pub rfid2: Option<String>,
}


