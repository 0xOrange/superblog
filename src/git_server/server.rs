pub mod git_server_routes {
    use rocket::http::RawStr;
    use rocket::*;

    pub fn get_routes() -> Vec<Route> {
        routes![get_info_refs, upload_pack, receive_pack]
    }

    #[get("/info/refs?<service>")]
    fn get_info_refs(service: &RawStr) -> String {
        format!("Got request: {}", service)
    }

    #[post("/git-upload-pack")]
    fn upload_pack() -> String {
        "TODO".into()
    }

    #[post("/git-receive-pack")]
    fn receive_pack() -> String {
        "TODO".into()
    }
}
