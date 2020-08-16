use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
pub struct GitRequest {
    command: String,
    repo: String,
}

impl GitRequest {
    pub fn new(cmd: &'static str) -> Result<GitRequest, String> {
        let r = Regex::new(
            r"^(git[-|\s]upload-pack|git[-|\s]upload-archive|git[-|\s]receive-pack) '(.*)'",
        )
        .map_err(|e| format!("{:?}", e))?;
        match r.captures_len() {
            3 => {
                let cap = r.captures(cmd).unwrap();
                Ok(GitRequest {
                    command: cap[1].to_string(),
                    repo: cap[2].to_string().replacen("/", "", 1),
                })
            }
            x => Err(format!("Invalid git command: {}", x)),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_git_repo() {
        use super::*;
        let commands = [
            (
                GitRequest::new("git-upload-pack 'test.git'"),
                GitRequest {
                    command: "git-upload-pack".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git upload-pack 'test.git'"),
                GitRequest {
                    command: "git upload-pack".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git-upload-pack '/test.git'"),
                GitRequest {
                    command: "git-upload-pack".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git-upload-pack '/hello/world.git'"),
                GitRequest {
                    command: "git-upload-pack".into(),
                    repo: "hello/world.git".into(),
                },
            ),
            (
                GitRequest::new("git-receive-pack '/test.git'"),
                GitRequest {
                    command: "git-receive-pack".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git receive-pack '/test.git'"),
                GitRequest {
                    command: "git receive-pack".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git-upload-archive '/test.git'"),
                GitRequest {
                    command: "git-upload-archive".into(),
                    repo: "test.git".into(),
                },
            ),
            (
                GitRequest::new("git upload-archive '/test.git'"),
                GitRequest {
                    command: "git upload-archive".into(),
                    repo: "test.git".into(),
                },
            ),
        ];
        for (req, result) in commands.iter() {
            println!("Checking for: {:?}", req);
            assert_eq!(req.as_ref().unwrap(), result);
        }
    }
}
