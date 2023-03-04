use tokio::sync::mpsc;
use crate::core::corex::CoreX;
use crate::logger;

impl CoreX {
    pub async fn run(&self) {
        let mut log = logger::get_logger();
        log.info(&format!("{} running", self.node_name));
        let (tx, mut rx) = mpsc::channel::<String>(32);
        rk_transporter::start_transporter(tx).await;
    }
}