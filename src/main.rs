#[macro_use]
extern crate rocket;
mod git_server;

#[get("/")]
fn index() -> &'static str {
    "Welcome!"
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
