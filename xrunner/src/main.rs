use rk_corex::core::corex::CoreX;
use rk_corex::logger;

#[tokio::main]
async fn main() {
    let mut log = logger::get_logger();
    let corex = CoreX::new("core-x".to_string());
    corex.init().await;
    // let agent = Agent::new(
    //     "0.0.1".to_string(),
    //     "test agent".to_string(),
    // );
    // let res = match corex.register(agent).await {
    //     Ok(res) => res,
    //     Err(err) => {
    //         log.error(&format!("error: {:?}", err));
    //         return;
    //     }
    // };
    log.info(&format!("agent registered: {:?}", corex.key()));
    corex.run().await;
}
