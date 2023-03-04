use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct AgentConfiguration {
    protocol: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Configuration {
    name: String,
    description: Option<String>,
    agent: AgentConfiguration,
}