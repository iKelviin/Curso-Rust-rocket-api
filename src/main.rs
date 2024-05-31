#[macro_use] extern crate rocket;

mod schema;
mod models;

use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::{response::status, serde::json::{json, Value}};
use rocket_sync_db_pools::database;
use schema::tbl_produtos;
use models::{NewProduto, Produto};
use rocket::serde::json::Json;

#[database("postgreSQL")]
struct DbConn(diesel::PgConnection);


#[get("/produtos")]
async fn get_produtos(_db: DbConn) -> Value {
    _db.run(|c| {
        let produtos = tbl_produtos::table
        .order_by(tbl_produtos::id.asc())
        .limit(1000)
        .load::<Produto>(c)
        .expect("DB Error");
    json!(produtos)
    }).await
}

#[get("/produtos/<id>")]
async fn get_produto(id: i64, _db: DbConn) -> Value {
    _db.run(move |c| {
        let produto = tbl_produtos::table.find(id)
            .get_result::<Produto>(c)
            .expect("Erro ao obter produto.");
    json!(produto)
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

#[put("/produtos/<id>", format = "json")]
fn put_produto(id: i64) -> Value {
    json!({"id": id, "name": "Kelvin","email":"kelvin.rocha@gmail.com"})
}

#[delete("/rustaprodutosceans/<_id>")]
fn delete_produto(_id: i64) -> status::NoContent {
    status::NoContent
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