use serde::Deserialize;

#[derive(Deserialize)]
pub struct RemoveQualificaQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveMansioneQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveOperaQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveTipoProprietaQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveImpresaQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveImpresaCollegataQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveUtenteQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveImpresaAssociateUtenteQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveDipendenteQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveMezzoQuery {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct RemoveAutovetturaQuery {
    pub id: i32,
}
