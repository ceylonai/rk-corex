use tokio::sync::mpsc;
use crate::core::corex::CoreX;
use crate::logger;

impl CoreX {
    pub async fn run(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} running", self.node_name));
        let (tx, mut rx) = mpsc::channel::<String>(32);
        tokio::task::spawn(async move {
            loop {
                let msg = rx.recv().await;
                log.info(&format!("Received: {:?}", msg));
                println!("Received: {:?}", msg);
            }
        });
        rk_transporter::start_transporter(tx, None).await;
    }
}