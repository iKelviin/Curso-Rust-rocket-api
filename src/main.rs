#[macro_use] extern crate rocket;

mod schema;
mod models;
mod lib;

use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{response::status, serde::json::{json, Value}};
use rocket_sync_db_pools::database;
use schema::tbl_produtos;
use models::{NewProduto, Produto};
use rocket::serde::json::Json;
use lib::MyUuid;
use crate::schema::tbl_produtos::dsl::*;
use log::info;

#[database("postgreSQL")]
struct DbConn(diesel::PgConnection);


#[get("/produtos")]
async fn get_produtos(_db: DbConn) -> Value {
    _db.run(|c| {
        let produtos = tbl_produtos::table
        .order_by(tbl_produtos::id.asc())
        .limit(20000)
        .load::<Produto>(c)
        .expect("DB Error");
    json!(produtos)
    }).await
}

#[get("/produtos/<_uuid>")]
async fn get_produto(_uuid: MyUuid, db: DbConn) -> Result<Json<Produto>, status::NotFound<String>> {
    db.run(move |c| {
        
        tbl_produtos.filter(uuid.eq(_uuid.0))
            .first::<Produto>(c)
            .map(Json)
            .map_err(|_| status::NotFound("Produto não encontrado".to_string()))
    }).await
}

#[post("/produtos", format = "json", data = "<new_produto>")]
async fn post_produto(_db: DbConn, new_produto: Json<NewProduto>) -> Value {
    _db.run(|c| {
        let result = diesel::insert_into(tbl_produtos::table)
            .values(new_produto.into_inner())
            .execute(c)
            .expect("Erro ao inserir no BD.");
        json!(result)
    }).await
}

/*
#[put("/produtos/<_uuid>", format = "json", data = "<produto>")]
async fn put_produto(_uuid: MyUuid, db: DbConn, produto: Json<Produto>) -> Value {
    db.run(move |c| {      
        use diesel::debug_query;
        use diesel::pg::Pg;

        let target = tbl_produtos.filter(uuid.eq(_uuid.0));
        info!("SQL Query: {:?}", debug_query::<Pg, _>(&target));        

        let updated_produto = diesel::update(target)
            .set(produto.into_inner())
            .execute(c)
            .expect("Erro ao atualizar produto no BD");
        json!(updated_produto)
    }).await
}*/

#[put("/produtos/<_id>", format = "json", data = "<produto>")]
async fn put_produto(_id: i64, db: DbConn, produto: Json<Produto>) -> Value {
    db.run(move |c| {                  
        let updated_produto = diesel::update(tbl_produtos::table.find(_id))
            .set(produto.into_inner())
            .execute(c)
            .expect("Erro ao atualizar produto no BD");
        json!(updated_produto)
    }).await
}

#[delete("/produtos/<_uuid>")]
async fn delete_produto(_uuid: MyUuid,_db: DbConn) -> status::NoContent {
    _db.run(move |c| {
        let target = tbl_produtos.filter(uuid.eq(_uuid.0));
        diesel::delete(target)
            .execute(c)
            .expect("Erro ao deletar produto no BD.");
        status::NoContent
    }).await
    
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

#[rocket::main]
async fn main() {
    let _= rocket::build()
    .mount("/",routes![
        get_produtos,
        get_produto,
        post_produto,
        put_produto,
        delete_produto
    ]).register("/", catchers![
        not_found
    ])
    .attach(DbConn::fairing())
    .launch().await;
}