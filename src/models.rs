use diesel::{Insertable, AsChangeset, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::tbl_produtos;

#[derive(Deserialize, Serialize, Queryable, AsChangeset)]
#[diesel(table_name = tbl_produtos)]
pub struct Produto{
    #[serde(skip_deserializing)]
    pub id: i64,
    #[serde(skip_deserializing)]
    pub uuid: Uuid,
    pub nome: String,
    pub ean: String,
    pub descricao: Option<String>,
    pub volume: String,
    pub departamento: String,
    pub categoria: String,
    pub subcategoria: String,
    pub imagemgrande: String,
    pub imagempequena: String
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = tbl_produtos)]
pub struct NewProduto{
    pub nome: String,
    pub ean: String,
    pub descricao: Option<String>,
    pub volume: String,
    pub departamento: String,
    pub categoria: String,
    pub subcategoria: String,
    pub imagemgrande: String,
    pub imagempequena: String
}