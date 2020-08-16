use std::fs;
use std::path::{Path, PathBuf};

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
        let hook_scripts = [
            ("pre-receive", &self.pre_receive),
            ("update", &self.update),
            ("post-receive", &self.post_receive),
        ];
        for (file_name, script) in hook_scripts.iter() {
            if let Some(s) = script {
                println!("Writing file: {}", file_name);
                fs::write(dir.join(file_name), s)
                    .map_err(|e| format!("Error writing file: {} - {:?}", file_name, e))?;
            }
        }

        Ok(())
    }

    fn get_hook_dir(repo_path: &Path) -> PathBuf {
        Path::new(repo_path).join(HookScripts::HOOKS_DIR_NAME)
    }

    fn create_hooks_dir(repo_path: &Path) -> Result<(), String> {
        let dir = Path::new(repo_path).join(HookScripts::HOOKS_DIR_NAME);
        match fs::create_dir(&dir) {
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod test {
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

        HookScripts::create_hooks_dir(repo_path).expect("Could not create hooks directory");
        assert!(dir.is_dir());
        // should ignore if directory already exists.
        HookScripts::create_hooks_dir(repo_path).expect("Could not create hooks directory");

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
