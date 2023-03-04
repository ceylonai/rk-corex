use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AgentConfiguation {
    protocol: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Configuation {
    name: String,
    description: Option<String>,
    agent: AgentConfiguation,
}