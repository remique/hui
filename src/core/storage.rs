use crate::core::{collection::Collection, config::Config};

struct StorageLayer {
    collections: Vec<Collection>,
    config: Config,
}

impl StorageLayer {
    fn new() -> Self {
        let config = Config::default();
        let collections = Vec::new();

        Self {
            collections,
            config,
        }
    }
}
