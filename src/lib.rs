use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse, Responder};

use std::{collections::HashMap, fs::File, sync::Mutex};

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    password: String,
    todo: Mutex<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Username{
    name : String,
}
// save the data to the database
fn update_db(data: &HashMap<String, User> , path: String){
    let new_data = File::create(path).unwrap();
    serde_json::to_writer(new_data, &data).expect("Failed to save data!!!");
}


// *******searching for user in database
// #[get("/{username}")] 
pub async fn user_status(username: Path<Username>, state: Data<HashMap<String, User>>) -> HttpResponse{
    let user = state.get(&username.name);
    match user{
        Some(user) => HttpResponse::Ok().json(format!("Welcome{}", username.name)),
        None => HttpResponse::Ok().json(format!("{} does not exist", username.name)),
    }
}

// ******display todolist
// #[get("/{username}/display")]
pub async fn display() -> HttpResponse{
    
}
// *******adding task to list
#[get("/{username}/add")]
pub async fn add(){

}
// ********deleting task from list
#[get("/{username}/delete")]
pub async fn delete(){

}

