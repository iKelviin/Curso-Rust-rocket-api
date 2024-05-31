use diesel::{prelude::Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::tbl_produtos;

#[derive(Serialize, Queryable)]
pub struct Produto{
    pub id: i64,
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