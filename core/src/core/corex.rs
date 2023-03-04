use conf::configuration_dto::Configuration;

use crate::{conf, logger};
use crate::agent::agent_structure::AgentKey;

pub struct CoreX {
    node_name: String,
    key: AgentKey,
}

impl CoreX {
    pub async fn init(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} initialized", self.node_name));

        let working_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        log.info(&format!("Working directory: {}", working_dir));
        Configuration::init_to_file(&format!("{}/configs/core-x.toml", working_dir));

        let config = Configuration::from_file(&format!("{}/configs/core-x.toml", working_dir));
        log.info(&format!("Configuration: {:?}", config));
    }
    pub async fn run(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} running", self.node_name));
    }
}

impl CoreX {
    pub fn new(name: String) -> CoreX {
        CoreX {
            node_name: name.clone(),
            key: AgentKey {
                domain: "core".to_string(),
                name,
            },
        }
    }

    pub fn key(&self) -> AgentKey {
        self.key.clone()
    }
}