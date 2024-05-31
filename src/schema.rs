// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "point", schema = "pg_catalog"))]
    pub struct Point;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Point;

    tbl_estabelecimentos (id) {
        id -> Int8,
        uuid -> Uuid,
        #[max_length = 255]
        nome -> Varchar,
        descricao -> Nullable<Text>,
        #[max_length = 255]
        categoria -> Varchar,
        coordenadas -> Nullable<Point>,
    }
}

diesel::table! {
    tbl_produtos (id) {
        id -> Int8,
        uuid -> Uuid,
        #[max_length = 255]
        nome -> Varchar,
        #[max_length = 20]
        ean -> Varchar,
        descricao -> Nullable<Text>,
        #[max_length = 100]
        volume -> Varchar,
        #[max_length = 255]
        departamento -> Varchar,
        #[max_length = 255]
        categoria -> Varchar,
        #[max_length = 255]
        subcategoria -> Varchar,
        #[max_length = 255]
        imagemgrande -> Varchar,
        #[max_length = 255]
        imagempequena -> Varchar,
    }
}

diesel::table! {
    tbl_produtosestabelecimentos (idproduto, idestabelecimento) {
        idproduto -> Int8,
        idestabelecimento -> Int8,
    }
}

diesel::joinable!(tbl_produtosestabelecimentos -> tbl_estabelecimentos (idestabelecimento));
diesel::joinable!(tbl_produtosestabelecimentos -> tbl_produtos (idproduto));

diesel::allow_tables_to_appear_in_same_query!(
    tbl_estabelecimentos,
    tbl_produtos,
    tbl_produtosestabelecimentos,
);
