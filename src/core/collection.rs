use crate::core::config::Config;

struct Request {
    url: String,
}

struct Collection {
    name: String,
    requests: Vec<Request>,
}

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
