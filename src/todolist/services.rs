use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, TodolistEntry};
use super::models::{CreateEntryData, UpdateEntryData};

#[get("todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}

#[post("todos")]
async fn create_todo(
    data: web::Data<AppState>, 
    params: web::Json<CreateEntryData>
) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id = 0;

    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    }

    todolist_entries.push(TodolistEntry{
        id: max_id + 1,
        title: params.title.clone(),
        date: params.date,
    });

    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[put("todos/{id}")]
async fn update_todo(
    data: web::Data<AppState>, 
    path: web::Path<i32>, 
    params: web::Json<UpdateEntryData>
) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();

    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = params.title.clone();
            break;
        }
    }
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

#[delete("todos/{id}")]
async fn delete_todo(
    data: web::Data<AppState>, 
    path: web::Path<i32>
) -> impl Responder {
    let id = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();

    *todolist_entries = todolist_entries.to_vec().into_iter().filter(|x| x.id != id).collect();
    HttpResponse::Ok().json(todolist_entries.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig){
    cfg
        .service(get_todos)
        .service(create_todo)
        .service(update_todo)
        .service(delete_todo);
}