use paris::Logger;

pub fn get_logger<'a>() -> Logger<'a> {
    let log = Logger::new();
    log
}