use actix_web::{post, web::{Data, Json, Path}, HttpResponse, Responder};

use std::{collections::HashMap, fs::File, sync::Mutex};

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User{
    pass: String,
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
        Some(_user) => HttpResponse::Ok().json(format!("Welcome{}", username.name)),
        None => HttpResponse::Ok().json(format!("{} does not exist", username.name)),
    }
}

// ******display todolist
// #[get("/{username}/display")]
pub async fn display(username: Path<Username>, state: Data<HashMap<String, User>>) -> impl Responder{
    let user = state.get(&username.name);
    match user{
        Some(_user) => HttpResponse::Ok().json(format!("Here is your ToDo_List: {:?}", &_user.todo)),
        None => HttpResponse::Ok().body("you have no Task to do!!!"),
    }
}
// *******adding task to list
#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub struct Add_task{
    task: String,
}
#[post("/{username}/add")]
pub async fn add(task: Json<Add_task>, username: Path<Username>, state: Data<HashMap<String, User>>) -> impl Responder{
    let user = state.get(&username.name);
    let todo = task.into_inner();
    match user{
        Some(x) => {
            {
                let mut new_task = x.todo.lock().unwrap();
                new_task.push(todo.task);
            }
            update_db(&state, String::from("database.json"));
            HttpResponse::Ok().json(format!("your updated todo list is: {:?}",&x.todo))
        }
        None => HttpResponse::Ok().json(format!("The User_Name {} does not exist.", username.name)),
    }
}
// ********deleting task from list)
#[derive(Serialize, Deserialize)]
pub struct DelTask{
    indx: usize,
}
#[post("/{username}/delete")]
pub async fn delete(task: Json<DelTask>, username: Path<Username>, state:Data<HashMap<String, User>>) -> impl Responder{
    let user = state.get(&username.name);
    let todo = task.into_inner();
    match user{
        Some(_user) => {
            {
                let mut new_task = _user.todo.lock().unwrap();
                new_task.remove(todo.indx);
            }
            update_db(&state, String::from("database.json"));
            HttpResponse::Ok().json(format!("Your new updated todo list is: {:?}", &_user.todo))
        }
        None => HttpResponse::Ok().json(format!("The User_name {} does not exist.", &username.name)),
    }
    
}

