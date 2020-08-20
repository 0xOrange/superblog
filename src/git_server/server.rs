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
        let info = format!("# service={}\n", self.git_rpc.value());
        let resp = format!(
            "{:04x}{}0000{}",
            info.len() + 4,
            info,
            String::from_utf8_lossy(if self.output.status.success() {
                &self.output.stdout
            } else {
                &self.output.stderr
            })
        );

        Response::build()
            .raw_header(
                "Content-Type",
                format!("application/x-{}-advertisement", self.git_rpc.value()),
            )
            .raw_header("Cache-Control", "no-cache")
            .sized_body(resp.len(), Cursor::new(resp))
            .ok()
    }
}

pub mod git_server_routes {
    use super::handler;
    use rocket::data::{Data, ToByteUnit};
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

    #[post("/git-upload-pack", data = "<content>")]
    async fn upload_pack(content: Data) -> Option<String> {
        let c = content
            .open(2u8.mebibytes())
            .stream_to_string()
            .await
            .ok()?;
        println!("Got content: {:?}", c);
        Some("TODO".into())
    }

    #[post("/git-receive-pack", data = "<content>")]
    async fn receive_pack(content: Data) -> Option<String> {
        let c = content.open(2u8.mebibytes()).stream_to_string().await;

        println!("Got content: {:?}", c);
        Some("TODO".into())
    }
}
