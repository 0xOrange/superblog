pub struct HookScripts {
    pub pre_receive: String,
    pub update: String,
    pub post_receive: String,
}

impl HookScripts {
    /// This will configure hook scripts in the repo base directory by writing them to the directory.
    pub fn setup_hooks(scripts: HookScripts) -> Result<(), String> {
        todo!()
    }
}
