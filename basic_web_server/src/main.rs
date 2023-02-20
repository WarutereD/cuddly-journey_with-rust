#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use serde::{Deserialize, Serialize};


#[derive(Serialize)]
struct ToDoList{
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem{
    id: i64,
    item: String,
}

#[derive(Serialize)]
struct StatusMessage{
    message: String,
}


#[get("/")]
fn index() -> &'static str{
    "Hello,world!"
}

fn main() {
    {
    let db_connection = rusqlite::Connection::open("data.sqlite").unwrap();

    db_connection
    .execute(
        "create table if not exists todo_list (
                id interger primary key,
                item varchar(64) not null
        );",
            [],
    )
     .unwrap();
    }


    rocket::ignite().mount("/", routes![index]).launch();

}
