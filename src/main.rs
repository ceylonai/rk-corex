use crate::agent::agent_manager::{Agent, AgentManager};
use crate::core::corex::CoreX;

mod core;
pub mod agent;

#[async_std::main]
async fn main() {
    let mut corex = CoreX::new("corex".to_string());
    corex.init().await;


    let agent = Agent::new(
        "0.0.1".to_string(),
        "test agent".to_string(),
    );
    let res = match corex.register(agent).await {
        Ok(res) => res,
        Err(err) => {
            println!("err: {:?}", err);
            return;
        }
    };
    println!("res: {:?}", res.key());
}