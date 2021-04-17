use postgres::error::Error;
use postgres::NoTls;
use postgres::Row;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::PostgresConnectionManager;

use std::env;

use crate::model::Note;

pub fn get_database_url() -> String {
    env::var("PG_URL").expect("PG_URL must be set")
}

pub fn get_pool() -> Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(get_database_url().parse().unwrap(), NoTls);
    let pool_size: u32 = env::var("PG_POOL_SIZE").expect("PG_POOL_SIZE must be set").parse::<u32>().unwrap();
    Pool::builder().max_size(pool_size).build(manager).unwrap()
}

pub fn insert_note(note: &Note, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let statement = db
        .prepare(
            "insert into notes (title, status) values ($1, $2)",
        )?;
    db.query(&statement, &[&note.title, &note.status])
}

pub fn read_notes(db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Note>, Error> {
    let statement = db
        .prepare(
            "select * from notes",
        )?;
    let notes: Vec<Note> = db.query(&statement, &[])?
        .iter()
        .map(|row| {
            let title: String = row.get("title");
            let status: String = row.get("status");
            Note {
                title,
                status,
            }
        }).collect();
    Ok(notes)
}

pub fn read_note(title: String, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Option<Note>, Error> {
    let statement = db
        .prepare(
            "select * from notes where title = $1 ",
        )?;

    let note: Option<Note> = db.query(&statement, &[&title])?
        .iter()
        .fold(None, |_acc, row| {
            let title: String = row.get("title");
            let status: String = row.get("status");
            Some(Note {
                title,
                status,
            })
        });
    Ok(note)
}

pub fn delete_note(title: String, db: &mut PooledConnection<PostgresConnectionManager<NoTls>>) -> Result<Vec<Row>, Error> {
    let statement = db
        .prepare(
            "delete from notes where title = $1",
        )?;
    db.query(&statement, &[&title])
}
