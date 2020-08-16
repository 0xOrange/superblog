mod hooks;
mod parser;

use std::fs;
use std::path::PathBuf;

pub struct GitServerConfig {
    pub dir: PathBuf,
    pub git_path: String,
    pub hooks: hooks::HookScripts,
}
impl Default for GitServerConfig {
    fn default() -> Self {
        GitServerConfig {
            dir: "./data/repo".into(),
            git_path: "/usr/bin/git".into(),
            hooks: hooks::HookScripts::default(),
        }
    }
}

impl GitServerConfig {
    pub fn setup(&self) -> Result<(), String> {
        // Create the repo directory
        fs::create_dir_all(&self.dir).map_err(|_| "Could not create git repo directory")?;
        Ok(())
    }
}

pub use parser::GitRequest;
