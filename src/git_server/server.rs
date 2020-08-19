use super::handler;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;
use std::process::Output;

struct GitOutput {
    pub git_rpc: handler::GitRpc,
    pub output: Output,
}

impl<'r> Responder<'r, 'static> for GitOutput {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .raw_header(
                "Content-Type",
                format!("application/x-{}-advertisement", self.git_rpc.value()),
            )
            .raw_header("Cache-Control", "no-cache")
            .streamed_body(Cursor::new(self.output.stdout))
            .ok()
    }
}

pub mod git_server_routes {
    use super::handler;
    use rocket::*;

    pub fn get_routes() -> Vec<Route> {
        routes![get_info_refs, upload_pack, receive_pack]
    }

    #[get("/info/refs?<service>")]
    async fn get_info_refs(service: handler::GitRpc) -> Result<super::GitOutput, String> {
        let git_handler = handler::GitHandler::default();
        let repo_name = "test-10";
        git_handler
            .create_repo(repo_name)
            .await
            .map_err(|e| format!("Could not create repo: {} - {:?}", repo_name, e))?;
        let cmd = git_handler
            .get_info_refs(&service, repo_name)
            .await
            .map_err(|e| {
                println!("Git error: {:?}", e);
                "Git Error."
            })?;
        Ok(super::GitOutput {
            git_rpc: service,
            output: cmd,
        })
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
