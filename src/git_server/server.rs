use super::handler::GitRpc;

pub mod git_server_routes {
    use super::GitRpc;
    use rocket::*;

    pub fn get_routes() -> Vec<Route> {
        routes![get_info_refs, upload_pack, receive_pack]
    }

    #[get("/info/refs?<service>")]
    fn get_info_refs(service: GitRpc) -> Option<String> {
        println!("Service: {:?}", service);
        Some(format!("Got request: {:?}", service))
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
