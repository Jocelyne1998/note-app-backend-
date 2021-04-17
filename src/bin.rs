#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use lib::db;
use lib::db::get_pool;
use lib::model::{Note, Storage};
use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;

fn main() {
    rocket().launch();
}

#[get("/")]
fn get_notes(state: State<Storage>) -> Json<Option<Vec<Note>>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_notes(&mut db).ok())
}

#[get("/<title>")]
fn get_note(title: &RawStr, state: State<Storage>) -> Json<Option<Note>> {
    let mut db = state.database.get().unwrap();
    Json(db::read_note(title.url_decode().expect("Failed to decode title."), &mut db).ok().flatten())
}

#[post("/", data = "<note>")]
fn create_note(note: Json<Note>, state: State<Storage>) -> Json<Option<Note>> {
    let mut db = state.database.get().unwrap();
    let inserted_note: Option<Note> = db::insert_note(&note.0, &mut db)
        .ok()
        .map(|_| note.0);
    Json(inserted_note)
}

#[delete("/<title>")]
fn delete_note(title: &RawStr, state: State<Storage>) -> Json<bool> {
    let mut db = state.database.get().unwrap();
    let parsed_title = title.url_decode().expect("Failed to decode title.");
    let result: bool = db::delete_note(parsed_title, &mut db).is_ok();
    Json(result)
}

fn rocket() -> rocket::Rocket {
    let database = get_pool();
    let storage = Storage { database };
    rocket::ignite().mount(
        "/notes",
        routes![get_notes, get_note, create_note, delete_note],
    ).manage(storage)
}