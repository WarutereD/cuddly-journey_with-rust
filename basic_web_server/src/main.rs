#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str{
    "Hello,world!"
}

fn main() {
    let db_connection = rusqlite::Connection::open("data.sqlite").unwrap();

    db_connection.execute(
        "create table if not exist test (id interger primary key);",[],
    )
     .unwrap();

    rocket::ignite().mount("/", routes![index]).launch();

}
