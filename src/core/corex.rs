pub struct CoreX {}

impl CoreX {
    pub fn new() -> CoreX {
        CoreX {}
    }

    pub async fn init(&self) {
        println!("CoreX: init");
    }

    pub async fn register_agent(&self) {
        println!("CoreX: register_agent");
    }
}