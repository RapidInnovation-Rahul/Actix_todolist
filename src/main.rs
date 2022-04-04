use actix_web::{web, App, HttpServer};
use std::io::BufReader;
use std::{collections::HashMap, fs::File};
use serde_json::Result;

mod lib;
use lib::{User,user_status};

fn read_data(path: String) -> Result<HashMap<String,User>>{
    let file = File::open(path).expect("failed to read data from the database!!");
    let reader = BufReader::new(file);
    let database: HashMap<String,User> = serde_json::from_reader(reader)?;
    
    Ok(database)
}
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let database = read_data(String::from("database.json")).unwrap();
    let database = web::Data::new(database);
    HttpServer::new(|| {
        App::new()
            .route("/{username}", web::get().to(user_status))
            .route("/{username}", web::get().to(display))
            .service(lib::add)
            .service(lib:: delete)

    })
    .bind("localhost:8080")?
    .bind("localhost:8080")?
    .bind("localhost:8080")?
    .run()
    .await
}