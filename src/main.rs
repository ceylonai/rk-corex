use crate::core::corex::CoreX;

mod core;

#[async_std::main]
async fn main() {
    let corex = CoreX::new();
    corex.init().await;

    corex.register_agent().await;
}