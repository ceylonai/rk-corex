mod run_method;

use conf::configuration_dto::Configuration;

use crate::{conf, logger};
use crate::agent::agent_structure::AgentKey;

pub struct CoreX {
    node_name: String,
    key: AgentKey,
    config: Configuration,
    config_dir: String,
}

impl CoreX {
    pub async fn init(&self) {
        let mut log = logger::get_logger();

        log.info(&format!("Configuration: {:?}", self.config));
        log.info(&format!("{} initialized", self.node_name));
    }
}

impl CoreX {
    pub fn new(name: String) -> CoreX {
        let mut log = logger::get_logger();
        let working_dir = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        let config_dir = format!("{working_dir}/configs");

        log.info(&format!("Working directory: {working_dir}"));
        let config = Configuration::init_configs(&config_dir);


        CoreX {
            node_name: name.clone(),
            key: AgentKey {
                domain: "core".to_string(),
                name,
            },
            config,
            config_dir,
        }
    }

    pub fn key(&self) -> AgentKey {
        self.key.clone()
    }
}