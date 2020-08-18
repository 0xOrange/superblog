mod git_server;
use rocket::*;

#[get("/")]
async fn hello() -> String {
    "Hello, world!".into()
}

#[launch]
fn launch() -> rocket::Rocket {
    let mut routes = routes![hello];
    routes.extend(git_server::routes::get_routes());
    rocket::ignite().mount("/", routes)
}
