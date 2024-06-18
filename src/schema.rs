// @generated automatically by Diesel CLI.

diesel::table! {
    autovettures (id) {
        id -> Int4,
        descrizione -> Nullable<Text>,
        modello -> Text,
        targa -> Text,
        tipo_proprieta -> Nullable<Int4>,
        #[max_length = 255]
        proprieta -> Varchar,
        impresa_id -> Nullable<Int4>,
        data_dimissioni -> Date,
        #[max_length = 50]
        rfid1 -> Varchar,
        #[max_length = 50]
        rfid2 -> Varchar,
    }
}

diesel::table! {
    dipendentis (id) {
        id -> Int4,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 255]
        cognome -> Varchar,
        #[max_length = 255]
        matricola -> Nullable<Varchar>,
        data_di_nascita -> Date,
        luogo_di_nascita -> Text,
        #[max_length = 255]
        codice_fiscale -> Varchar,
        impresa_id -> Nullable<Int4>,
        qualifica -> Nullable<Int4>,
        mansione -> Nullable<Int4>,
        data_dimissioni -> Date,
        #[max_length = 50]
        rfid1 -> Varchar,
        #[max_length = 50]
        rfid2 -> Varchar,
    }
}

diesel::table! {
    impreses (id) {
        id -> Int4,
        ragion_sociale -> Nullable<Text>,
        indirizzo -> Text,
        targa -> Text,
        #[max_length = 255]
        partita_iva -> Varchar,
        proprieta -> Text,
        data_dimissioni -> Date,
        #[max_length = 50]
        rfid1 -> Varchar,
        #[max_length = 50]
        rfid2 -> Varchar,
    }
}

diesel::table! {
    imprese_associate_utentis (id) {
        id -> Int4,
        utente_id -> Nullable<Int4>,
        impresa_id -> Nullable<Int4>,
    }
}

diesel::table! {
    imprese_collegates (id) {
        id -> Int4,
        impresa_id -> Nullable<Int4>,
        #[max_length = 50]
        ruolo_impresa -> Varchar,
    }
}

diesel::table! {
    mansionis (id) {
        id -> Int4,
        descrizione -> Text,
    }
}

diesel::table! {
    mezzis (id) {
        id -> Int4,
        descrizione -> Nullable<Text>,
        modello -> Text,
        tipo_proprieta -> Nullable<Int4>,
        #[max_length = 255]
        proprieta -> Varchar,
        impresa_id -> Nullable<Int4>,
        data_dimissioni -> Date,
        #[max_length = 50]
        rfid1 -> Varchar,
        #[max_length = 50]
        rfid2 -> Varchar,
    }
}

diesel::table! {
    operes (id) {
        id -> Int4,
        descrizione -> Text,
    }
}

diesel::table! {
    qualifiches (id) {
        id -> Int4,
        descrizione -> Text,
    }
}

diesel::table! {
    tipi_proprietas (id) {
        id -> Int4,
        descrizione -> Text,
    }
}

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
    utentis (id) {
        id -> Int4,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 255]
        cognome -> Varchar,
        impresa_id -> Nullable<Int4>,
        #[max_length = 255]
        utente -> Varchar,
        autorizazzione -> Bool,
    }
}

diesel::joinable!(autovettures -> impreses (impresa_id));
diesel::joinable!(autovettures -> tipi_proprietas (tipo_proprieta));
diesel::joinable!(dipendentis -> impreses (impresa_id));
diesel::joinable!(dipendentis -> mansionis (mansione));
diesel::joinable!(dipendentis -> qualifiches (qualifica));
diesel::joinable!(imprese_associate_utentis -> impreses (impresa_id));
diesel::joinable!(imprese_associate_utentis -> utentis (utente_id));
diesel::joinable!(imprese_collegates -> impreses (impresa_id));
diesel::joinable!(mezzis -> impreses (impresa_id));
diesel::joinable!(mezzis -> tipi_proprietas (tipo_proprieta));
diesel::joinable!(utentis -> impreses (impresa_id));

diesel::allow_tables_to_appear_in_same_query!(
    autovettures,
    dipendentis,
    impreses,
    imprese_associate_utentis,
    imprese_collegates,
    mansionis,
    mezzis,
    operes,
    qualifiches,
    tipi_proprietas,
    users,
    utentis,
);
