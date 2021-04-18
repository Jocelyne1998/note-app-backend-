#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db;
use lib::db::get_pool;
use lib::model::{Note, Storage};
use rocket::State;
use rocket::http::RawStr;
use rocket_contrib::json::Json;
use serde::{Deserialize};

fn main() {
    rocket().launch();
}

#[derive(Deserialize)]
struct NewNote {
    title: String
}

#[get("/")]
fn get_notes(state: State<Storage>) -> Json<Option<Vec<Note>>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_notes(&mut db).ok())
}

#[get("/<status>")]
fn get_note(status: &RawStr, state: State<Storage>) -> Json<Option<Vec<Note>>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_note(status.url_decode().expect("Failed to decode status."), &mut db).ok())
} 

#[post("/", data = "<note>")]
fn create_note(note: Json<NewNote>, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    let inserted_note: bool = db::insert_note(note.title.to_string(), &mut db).is_ok();
    Json(inserted_note)
}

#[post("/update", data = "<note>")]
fn update_note(note: Json<NewNote>, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    let updated_note: bool = db::modify_note(note.title.to_string(), &mut db).is_ok();
    Json(updated_note)
} 

#[get("/delete")]
fn delete_notes(state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    let result: bool = db::delete_notes(&mut db).is_ok();
    Json(result)
}

fn rocket() -> rocket::Rocket {
    let database = get_pool();
    let storage = Storage { database };
    rocket::ignite().mount(
        "/notes",
        routes![get_notes, update_note, create_note, delete_notes, get_note],
    ).manage(storage)
}