#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return "hello";
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])  // set routing
        .launch();
}