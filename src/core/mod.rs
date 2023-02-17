pub mod corex;

pub trait AgentManager {
    fn send(&self, msg: &str);
}

