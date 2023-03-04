use tokio::sync::mpsc;
use rk_corex::logger;

#[tokio::main]
async fn main() {
    let mut log = logger::get_logger();
    let (tx, mut rx) = mpsc::channel::<String>(32);
    tokio::task::spawn(async move {
        loop {
            let msg = rx.recv().await;
            log.info(&format!("Received Client: {:?}", msg));
            println!("Received Client: {:?}", msg);
        }
    });
    rk_transporter::start_transporter(tx, Some("/ip4/192.168.122.1/tcp/43349".to_string())).await;
}
