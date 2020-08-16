mod hooks;
mod parser;

pub struct GitServerConfig {
    pub dir: String,
    pub git_path: String,
    pub hooks: hooks::HookScripts,
}
impl Default for GitServerConfig {
    fn default() -> Self {
        GitServerConfig {
            dir: "./".into(),
            git_path: "/usr/bin/git".into(),
            hooks: hooks::HookScripts {
                pre_receive: r#"echo "pre-receive""#.into(),
                update: r#"echo "update""#.into(),
                post_receive: r#"echo "post-receive""#.into(),
            },
        }
    }
}

pub use parser::GitRequest;
