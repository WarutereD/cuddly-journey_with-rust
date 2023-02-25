             #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use serde::Serialize;
use rusqlite::{Connection, Result};
use std::error::Error;


#[derive(Debug, Serialize)]
struct ToDoList{
    items: Vec<ToDoItem>,
}

#[derive(Debug, Serialize)]
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

#[get("/todo")]
fn fetch_all_todo_items() -> Result<Json<ToDoList>, Box<dyn Error>> {
    let conn = Connection::open("data.sqlite").map_err(|_| format!("Failed to connect to database"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo_list (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            item VARCHAR(64) NOT NULL
        )",
        [],
    )
    .map_err(|err| format!("Failed to create table: {}", err))?;

    let mut statement = conn.prepare("SELECT id, item FROM todo_list").map_err(|err| format!("Failed to prepare query: {}", err.description()))?;

    let rows = statement.query_map([], |row| {
        Ok(ToDoItem {
            id: row.get(0)?,
            item: row.get(1)?,
        })
    }).map_err(|err| format!("Failed to fetch todo items: {}", err.description()))?;

    let items: Vec<_> = rows.collect::<Result<Vec<_>, _>>().map_err(|err| format!("Could not collect items: {}", err.description()))?;

    Ok(Json(ToDoList { items }))
}




#[post("/todo", format = "json", data = "<item>")]
fn add_todo_items(item: Json<String>) -> Result<Json<StatusMessage>, String>{
    let db_connection = match Connection::open("data.sqlite"){
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match  db_connection
        .prepare("insert into todo_list (id, item) values(null, $1);"){
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&item.0]);
        

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage{ 
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to insert todo items".into()),
    }
}

#[delete("/todo/<id>")]
fn remove_todo_items(id: i64) -> Result<Json<StatusMessage>, String>{
    let db_connection = match Connection::open("data.sqlite"){
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match  db_connection
        .prepare("delete from todo_list where id = $1;"){
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&id]);
        

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage{ 
            message: format!("{} rows Deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete todo items".into()),
    }
}

fn main() {
   // {
    //let db_connection = Connection::open("data.sqlite").unwrap();

    //db_connection
    //.execute(
      //  "create table if not exists todo_list (
        //        id interger primary key,
          //      item varchar(64) not null
        //);",
          //  [],
    //)
     //.unwrap();
    //}


    rocket::ignite().mount("/", routes![index, fetch_all_todo_items, add_todo_items, remove_todo_items]).launch();

}
