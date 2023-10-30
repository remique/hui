use crate::core::{collection::Collections, config::Config};

struct App {
    collections: Collections,
    config: Config,
}

impl App {
    fn new() -> Self {
        let config = Config::default();
        let collections = Collections::from_str("").unwrap();

        Self {
            collections,
            config,
        }
    }
}
