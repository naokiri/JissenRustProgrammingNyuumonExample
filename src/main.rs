mod schema;
mod models;

#[macro_use]
extern crate diesel;

use serde::Deserialize;

use actix_web::{HttpResponse, HttpServer, App, ResponseError, web, get, post};
use thiserror::Error;
use askama::Template;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use crate::models::TodoEntry;
use actix_web::http::header;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>
}

#[derive(Error, Debug)]
enum MyError {
    #[error("Failed to render HTML")]
    AskamaError(#[from] askama::Error),

    #[error("Failed to get connection")]
    ConnectionPoolError(#[from] diesel::r2d2::PoolError),

    #[error("Internal DB server error")]
    InternalDBError(#[from] diesel::result::Error),
}

impl ResponseError for MyError {}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let manager = ConnectionManager::<SqliteConnection>::new("todo.db");
    let pool = Pool::builder().build(manager).expect("Pool initialization error");


    HttpServer::new(move ||
        App::new()
            .service(index)
            .service(add_todo)
            .data(pool.clone()))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index(db: web::Data<Pool<ConnectionManager<SqliteConnection>>>) -> Result<HttpResponse, MyError> {
    use schema::todo::dsl::*;
    use schema::todo::all_columns;

    let conn = db.get()?;
    let entries = todo.select(all_columns).load::<TodoEntry>(&conn)?;


    let html = IndexTemplate { entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[post("/add")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<Pool<ConnectionManager<SqliteConnection>>>,
) -> Result<HttpResponse, MyError> {
    use schema::todo::dsl::*;

    let conn = db.get()?;
    diesel::insert_into(todo).values(text.eq(&params.text)).execute(&conn)?;
    Ok(HttpResponse::SeeOther().header(header::LOCATION, "/").finish())
}
