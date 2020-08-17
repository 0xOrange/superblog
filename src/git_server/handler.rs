use super::hooks;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GitHandler {
    pub dir: PathBuf,
    pub git_path: String,
    pub hooks: hooks::HookScripts,
}
impl Default for GitHandler {
    fn default() -> Self {
        GitHandler {
            dir: Path::new("./data/repo").into(),
            git_path: "git".into(),
            hooks: hooks::HookScripts::default(),
        }
    }
}

impl GitHandler {
    pub fn create_repo(&self, name: &str) -> Result<(), String> {
        let repo_path = &self.init_repo_path(name).map_err(|e| {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                "This repository already exists".into()
            } else {
                format!("An error occurred {:?}", e)
            }
        })?;
        let status = Command::new("git")
            .arg("init")
            .arg("--bare")
            .current_dir(repo_path)
            .status()
            .expect("failed to init repo");
        println!("git init exit status: {}", status);
        if status.success() {
            Ok(())
        } else {
            Err("Error init repo: {}".into())
        }
    }

    /// Create the directory for the repository.
    fn init_repo_path(&self, repo_name: &str) -> Result<PathBuf, std::io::Error> {
        if !&self.dir.is_dir() {
            fs::create_dir(&self.dir)?
        }

        let path = Path::new(&self.dir).join(repo_name);
        fs::create_dir(&path)?;
        Ok(path)
    }
}

#[cfg(test)]
mod test {
    use super::super::hooks;
    use super::GitHandler;
    use std::fs;
    use std::path::Path;

    fn test_setup(repo_path: &str) -> GitHandler {
        GitHandler {
            dir: Path::new(repo_path).into(),
            git_path: "git".into(),
            hooks: hooks::HookScripts::default(),
        }
    }

    fn cleanup(repo_path: &str) {
        fs::remove_dir_all(repo_path).unwrap();
    }

    #[test]
    fn init_repo() -> Result<(), String> {
        let repo_path = "test2";
        let repo_name = "repo1";
        let git_handler = test_setup(repo_path);

        git_handler.create_repo(repo_name)?;
        assert!(Path::new(repo_path).join(repo_name).join("HEAD").is_file());

        cleanup(repo_path);
        Ok(())
    }
}
