-- Your SQL goes here

CREATE TABLE qualifiches (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE mansionis (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE operes (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE tipi_proprietas (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE impreses (
    id SERIAL PRIMARY KEY,
    ragione_sociale TEXT NOT NULL,
    indirizzo TEXT NOT NULL,
    partita_iva VARCHAR(255) NOT NULL
);

CREATE TABLE imprese_collegates (
    id SERIAL PRIMARY KEY,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    imprese_collegata_id INT NOT NULL,
    ruolo_impresa VARCHAR(50) NOT NULL
);


CREATE TABLE utentis (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    nome VARCHAR(255) NOT NULL,
    cognome VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    utente VARCHAR(255) NOT NULL,
    autorizazzione BOOLEAN DEFAULT FALSE,
    primo_login BOOLEAN DEFAULT TRUE,
    super_utente BOOLEAN DEFAULT FALSE
);

CREATE TABLE imprese_associate_utentis (
    id SERIAL PRIMARY KEY,
    utente_id INT REFERENCES utentis(id) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL
);

CREATE TABLE dipendentis (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    cognome VARCHAR(255) NOT NULL,
    matricola VARCHAR(255),
    data_di_nascita DATE NOT NULL,
    luogo_di_nascita TEXT NOT NULL,
    codice_fiscale VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    qualifica INT REFERENCES qualifiches(id) NOT NULL,
    mansione INT REFERENCES mansionis(id) NOT NULL,
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE mezzis (
    id SERIAL PRIMARY KEY,
    descrizione TEXT ,
    modello TEXT NOT NULL,
    tipo_proprieta INT REFERENCES tipi_proprietas(id) NOT NULL,
    proprieta VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE autovettures (
    id SERIAL PRIMARY KEY,
    descrizione TEXT,
    modello TEXT NOT NULL,
    targa TEXT NOT NULL,
    tipo_proprieta INT REFERENCES tipi_proprietas(id) NOT NULL,
    proprieta VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE settimanales (
    id SERIAL PRIMARY KEY,
    data_settimanale TEXT NOT NULL,
    utente_id INT REFERENCES utentis(id) NOT NULL,
    luogo_di_nascita TEXT NOT NULL,
    data_di_nascita TEXT NOT NULL,
    tipo_proprieta INT REFERENCES tipi_proprietas(id) NOT NULL,
    proprieta VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES impreses(id) NOT NULL,
    opera_id DATE NOT NULL,
    mezzo_id INT REFERENCES mezzis(id) NOT NULL,
    autovettura_id INT REFERENCES autovettures(id) NOT NULL,
    matricola VARCHAR(50) NOT NULL,
    targa VARCHAR(50) NOT NULL
);


INSERT INTO "impreses" ("ragione_sociale", "partita_iva", "indirizzo") VALUES ('super', 'super', 'super');
INSERT INTO "utentis" ("username", "password", "nome", "cognome", "impresa_id", "utente", "primo_login", "super_utente") VALUES ('super', 'super', 'super', 'super', 1, 'super', FALSE, TRUE);
