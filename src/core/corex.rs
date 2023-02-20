use async_trait::async_trait;
use crate::agent::agent_manager::{Agent, AgentError, AgentKey, AgentManager, AgentStore};
use crate::logger;

pub struct CoreX {
    node_name: String,
    agent_store: AgentStore,
}

impl CoreX {
    pub fn new(name: String) -> CoreX {
        CoreX {
            agent_store: AgentStore::new(name.clone()),
            node_name: name.clone(),
        }
    }

    pub async fn init(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} initialized", self.node_name));
    }
}

#[async_trait]
impl AgentManager for CoreX {
    async fn register(&mut self, agent: Agent) -> Result<AgentKey, AgentError> {
        let key = self.agent_store.register(agent);
        Ok(key)
    }
}