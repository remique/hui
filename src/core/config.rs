pub struct Config {
    work_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        // TODO: By default this should be something like $HOME/.config/collections/
        // And should take operating system into account
        Self {
            work_dir: "./collections/".to_string(),
        }
    }
}

// TODO: Add ConfigBuilder?
