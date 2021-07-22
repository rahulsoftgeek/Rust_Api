#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
#[macro_use] extern crate diesel;
//#[macro_use] extern crate diesel_migrations;

mod models;
mod schema;

use models::*;
use schema::*;
use diesel::prelude::*;
use rocket::serde::json::{Json, Value, json};

//embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/emp")]
async fn get_emp(conn: DbConn) -> Value {
    conn.run(|c| {
        let all = employees::table.limit(100).load::<Employee>(c).expect("Error loading from DB");
        json!(all)
    }).await 
}
#[get("/emp/<id>")]
async fn view_emp(id: i32, conn: DbConn) -> Value {
    conn.run(move |c| {
        let employee = employees::table.find(id)
        .get_result::<Employee>(c)
        .expect("Error loading Employee info from DB");
        json!(employee)
    }).await
}
#[post("/emp", format = "json", data="<new_employee>")]
async fn create_emp(conn: DbConn, new_employee: Json<NewEmployee>) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(employees::table)
        .values(new_employee.into_inner())
        .execute(c)
        .expect("Error adding employees to DB");
        json!(result)
    }).await
}
#[put("/emp/<id>", format = "json", data="<employee>")]
async fn update_emp(id: i32, conn: DbConn, employee: Json<Employee>) -> Value {
    conn.run(move |c| {
        let result = diesel::update(employees::table.find(id))
        .set((
            employees::email.eq(employee.email.to_owned()),
            employees::name.eq(employee.name.to_owned()),
        ))
        .execute(c)
        .expect("Error updating employees into DB");
        json!(result)
    }).await
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

#[catch(500)]
fn internal_server_error() -> Value {
    json!("500 Internal Server Error")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            get_emp,
            create_emp,
            view_emp,
            update_emp
        ])
        .register("/", catchers![
            not_found,
            internal_server_error
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
