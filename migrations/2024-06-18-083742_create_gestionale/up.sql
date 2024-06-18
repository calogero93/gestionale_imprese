-- Your SQL goes here

CREATE TABLE impresa (
    id SERIAL PRIMARY KEY,
    ragion_sociale TEXT,
    indirizzo TEXT NOT NULL,
    targa TEXT NOT NULL,
    partita_iva VARCHAR(255) NOT NULL,
    proprieta TEXT NOT NULL,
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE qualifica (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE mansione (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE opera (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE tipo_proprieta (
    id SERIAL PRIMARY KEY,
    descrizione TEXT NOT NULL
);

CREATE TABLE utente (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    nome VARCHAR(255) NOT NULL,
    cognome VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES imprese(id),
    utente VARCHAR(255) NOT NULL,
    autorizazzione BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE imprese_associate_utente (
    id SERIAL PRIMARY KEY,
    utente_id INT REFERENCES utenti(id),
    impresa_id INT REFERENCES imprese(id)
);

CREATE TABLE dipendente (
    id SERIAL PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    cognome VARCHAR(255) NOT NULL,
    matricola VARCHAR(255),
    data_di_nascita DATE NOT NULL,
    luogo_di_nascita TEXT NOT NULL,
    codice_fiscale VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES imprese(id),
    qualifica INT REFERENCES qualifiche(id),
    mansione INT REFERENCES mansioni(id),
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE mezzo (
    id SERIAL PRIMARY KEY,
    descrizione TEXT ,
    modello TEXT NOT NULL,
    tipo_proprieta INT REFERENCES tipi_proprieta(id),
    proprieta VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES imprese(id),
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);

CREATE TABLE autovettura (
    id SERIAL PRIMARY KEY,
    descrizione TEXT,
    modello TEXT NOT NULL,
    targa TEXT NOT NULL,
    tipo_proprieta INT REFERENCES tipi_proprieta(id),
    proprieta VARCHAR(255) NOT NULL,
    impresa_id INT REFERENCES imprese(id),
    data_dimissioni DATE NOT NULL,
    rfid1 VARCHAR(50) NOT NULL,
    rfid2 VARCHAR(50) NOT NULL
);


CREATE TABLE impresa_collegata (
    id SERIAL PRIMARY KEY,
    impresa_id INT REFERENCES imprese(id),
    ruolo_impresa VARCHAR(50) NOT NULL
);



