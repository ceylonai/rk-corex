use std::collections::HashMap;


use async_trait::async_trait;
use nanoid::nanoid;
use crate::agent::agent_structure::{Agent, AgentError, AgentKey};


pub struct AgentStore {
    domain: String,
    agents: HashMap<AgentKey, Agent>,
}

impl AgentStore {
    pub fn register(&mut self, agent: Agent) -> AgentKey {
        let key = self.get_next_agent_key();
        self.agents.insert(key.clone(), agent);
        key
    }
}

impl AgentStore {
    pub fn new(domain: String) -> AgentStore {
        AgentStore {
            agents: HashMap::new(),
            domain,
        }
    }

    /// # Calculate the next agent key.
    /// - The agent key is a combination of the domain and a random string.
    /// - The random string is generated using the nanoid crate.
    pub fn get_next_agent_key(&self) -> AgentKey {
        AgentKey {
            domain: self.domain.clone(),
            name: nanoid!(20),
        }
    }
}

#[async_trait]
pub trait AgentManager {
    async fn register(&mut self, agent: Agent) -> Result<AgentKey, AgentError>;
}