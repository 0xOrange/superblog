use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct GitHandler {
    pub dir: PathBuf,
    pub git_path: String,
    pub hooks: HookScripts,
}
impl Default for GitHandler {
    fn default() -> Self {
        GitHandler {
            dir: Path::new("./data/repo").into(),
            git_path: "git".into(),
            hooks: HookScripts::default(),
        }
    }
}

impl GitHandler {
    pub fn create_repo(&self, repo_name: &str) -> Result<(), String> {
        if self.repo_exists(&repo_name) {
            return Ok(());
        }

        // create the directory for the repository
        if !self.dir.is_dir() {
            fs::create_dir(&self.dir).map_err(|_| "Could not create the repo directory")?
        }

        let repo_path = Path::new(&self.dir).join(repo_name);

        if !repo_path.is_dir() {
            fs::create_dir(&repo_path).map_err(|e| format!("Error creating directory: {:?}", e))?;
        }

        let status = Command::new(&self.git_path)
            .arg("init")
            .arg("--bare")
            .current_dir(&repo_path)
            .status()
            .expect("failed to init repo");
        println!("git init exit status: {}", status);
        if status.success() {
            self.hooks.write_hook_files(&repo_path)?;
            Ok(())
        } else {
            Err("Error init repo: {}".into())
        }
    }

    pub fn get_info_refs(&self) -> Result<(), ()> {
        todo!()
    }

    pub fn repo_exists(&self, repo_name: &str) -> bool {
        self.dir.join(repo_name).join("objects").is_dir()
    }
}

pub struct HookScripts {
    pub pre_receive: Option<String>,
    pub update: Option<String>,
    pub post_receive: Option<String>,
}
impl Default for HookScripts {
    fn default() -> Self {
        HookScripts {
            pre_receive: Some(r#"echo "pre-receive""#.into()),
            update: Some(r#"echo "update""#.into()),
            post_receive: Some(r#"echo "post-receive""#.into()),
        }
    }
}

impl HookScripts {
    /// Directory for git hooks within the repo.
    const HOOKS_DIR_NAME: &'static str = "hooks";

    fn write_hook_files(&self, repo_path: &Path) -> Result<(), String> {
        let dir = HookScripts::get_hook_dir(repo_path);

        let _: Result<(), String> = match fs::create_dir(&dir) {
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            Err(e) => {
                return Err(format!(
                    "The hooks directory does not exist, could not create: {:?}",
                    e
                ))
            }
            _ => Ok(()),
        };

        let hook_scripts = [
            ("pre-receive", &self.pre_receive),
            ("update", &self.update),
            ("post-receive", &self.post_receive),
        ];
        for (file_name, script) in hook_scripts.iter() {
            if let Some(s) = script {
                println!("Writing hook file: {}", file_name);
                fs::write(dir.join(file_name), s)
                    .map_err(|e| format!("Error writing file: {} - {:?}", file_name, e))?;
            }
        }

        Ok(())
    }

    fn get_hook_dir(repo_path: &Path) -> PathBuf {
        Path::new(repo_path).join(HookScripts::HOOKS_DIR_NAME)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn test_setup(repo_path: &str) -> GitHandler {
        let _ = fs::remove_dir_all(repo_path);

        GitHandler {
            dir: Path::new(repo_path).into(),
            git_path: "git".into(),
            hooks: HookScripts::default(),
        }
    }

    fn cleanup(repo_path: &str) {
        fs::remove_dir_all(repo_path).unwrap();
    }

    #[test]
    fn repo_exists() {
        let repo_path = "test1";
        let repo_name = "repo1";
        let git_handler = test_setup(repo_path);

        assert!(!git_handler.repo_exists(repo_name));
        git_handler.create_repo(repo_name).unwrap();
        assert!(git_handler.repo_exists(repo_name));

        cleanup(repo_path);
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

    #[test]
    fn should_init_hooks() {
        use super::*;
        let repo_path = Path::new("./test-temp");

        // Init
        let _ = fs::remove_dir_all(repo_path);
        fs::create_dir_all(repo_path).expect("Could not create git repo directory");

        let dir = Path::new(repo_path).join(HookScripts::HOOKS_DIR_NAME);
        assert_eq!(dir, HookScripts::get_hook_dir(repo_path));
        assert!(!dir.is_dir());

        // Check if the files are created
        let expected_hook_scripts = HookScripts {
            update: None,
            ..HookScripts::default()
        };
        expected_hook_scripts
            .write_hook_files(repo_path)
            .expect("Failed to write hook files");

        let hook_scripts = [
            ("pre-receive", expected_hook_scripts.pre_receive),
            ("update", expected_hook_scripts.update),
            ("post-receive", expected_hook_scripts.post_receive),
        ];

        for (file_name, script) in hook_scripts.iter() {
            let file_path = HookScripts::get_hook_dir(repo_path).join(file_name);
            match script {
                Some(s) => {
                    let content = fs::read_to_string(&file_path)
                        .expect(&format!("Unable to read file: {}", file_name));
                    assert_eq!(content, *s)
                }
                None => {
                    // file should not be created
                    assert!(!&file_path.is_file())
                }
            }
        }

        // cleanup
        let _ = fs::remove_dir_all(repo_path);
    }
}
