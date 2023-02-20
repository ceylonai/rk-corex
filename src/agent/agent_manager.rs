use std::collections::HashMap;


use async_trait::async_trait;
use nanoid::nanoid;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct AgentKey {
    domain: String,
    name: String,
}

impl AgentKey {
    pub fn key(&self) -> String {
        format!("{}:{}", self.domain, self.name)
    }
}

#[derive(Debug)]
pub enum AgentError {
    AgentAlreadyExists,
}

pub struct Agent {
    key: Option<AgentKey>,
    version: String,
    description: String,
    active: bool,
}

impl Agent {
    pub fn new(version: String, description: String) -> Agent {
        Agent {
            key: None,
            version,
            description,
            active: true,
        }
    }
}

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