#[allow(dead_code)]
pub enum MsgLevel {
    Info,
    Error,
    Debug,
}

pub fn print(level: MsgLevel, msg: &str) {
    let level = match level {
        MsgLevel::Info => "INFO",
        MsgLevel::Error => "ERROR",
        MsgLevel::Debug => "DEBUG"
    };
    println!("[{}] {}", level, msg)
}
