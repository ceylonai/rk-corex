use crate::agent::agent_structure::AgentKey;
use crate::logger;

pub struct CoreX {
    node_name: String,
    key: AgentKey,
}

impl CoreX {
    pub async fn init(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} initialized", self.node_name));
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