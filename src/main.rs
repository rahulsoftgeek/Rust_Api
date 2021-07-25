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
use rocket::http::Status;
use rocket::request::{Request, FromRequest, Outcome};


//embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/emp")]
async fn get_emp(_auth:BasicAuth,conn: DbConn) -> Value {
    conn.run(|c| {
        let all = employees::table.limit(100).load::<Employee>(c).expect("Error loading from DB");
        json!(all)
    }).await 
}
#[get("/emp/<id>")]
async fn view_emp(id: i32, _auth:BasicAuth, conn: DbConn) -> Value {
    conn.run(move |c| {
        let employee = employees::table.find(id)
        .get_result::<Employee>(c)
        .expect("Error loading Employee info from DB");
        json!(employee)
    }).await
}
#[post("/emp", format = "json", data="<new_employee>")]
async fn create_emp(_auth:BasicAuth, conn: DbConn, new_employee: Json<NewEmployee>) -> Value {
    conn.run(|c| {
        let result = diesel::insert_into(employees::table)
        .values(new_employee.into_inner())
        .execute(c)
        .expect("Error adding employees to DB");
        json!(result)
    }).await
}
#[put("/emp/<id>", format = "json", data="<employee>")]
async fn update_emp(id: i32, _auth:BasicAuth, conn: DbConn, employee: Json<Employee>) -> Value {
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
#[catch(401)]
fn unauthorized() -> Value {
    json!("401 - UnAuthorized")
}

pub struct BasicAuth{
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split = decoded_str.split(":").collect::<Vec<_>>();

        // If exactly username & password pair are present
        if split.len() != 2 {
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth {
            username,
            password
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_authorization_header(auth_header) {
                if auth.username == String::from("foo") && auth.password == String::from("bar") {
                return Outcome::Success(auth)
                }
            }
        }
        
        Outcome::Failure((Status::Unauthorized, ()))
    }
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
            internal_server_error,
            unauthorized,
        ])
        .attach(DbConn::fairing())
        .launch()
        .await;
}
