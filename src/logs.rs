use chrono::prelude::*;

pub fn local_time () -> String {
    let local: DateTime<Local> = Local::now();
    let format_time = local.format("%F %T").to_string();
    format_time
}

pub struct Logger {
}

impl Logger {
    pub fn log(msg: &str) -> bool {
        let log_level = "INFO".to_string();
        let log_msg = format!("{} {} {}", local_time(), log_level, msg);
        println!("{:?}", log_msg);
        true
    }
    pub fn err(msg: &str) -> bool {
        let log_level = "ERROR".to_string();
        let log_msg = format!("{} {} {}", local_time(), log_level, msg);
        println!("{:?}", log_msg);
        true
    }
}

