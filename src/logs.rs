use super::functions::local_time_format;
use std::cmp::min;
use reqwest::{blocking, StatusCode, Response};
use std::collections::HashMap;
use std::time::SystemTime;


pub async fn single_report_log<'a>(level: &'a str, step: &'a str, log: &'a str) -> HashMap<&'a str, String> {
    let mut timestamp: u64 = 0;

    if let Ok(time) = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        timestamp = time.as_secs();
    };

    let log_hash = HashMap::from([
            ("step", step.to_string()),
            ("level", level.to_string()),
            ("log", log.to_string()),
            ("timestamp", timestamp.to_string()),
        ]);
    log_hash
}


fn report_log(url: &str, task_id: &str, token: &str, logs: &str) ->  Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("task_id", task_id);
    map.insert("token", token);
    map.insert("logs", logs);
    let client = blocking::Client::new();
    let res = client.post(url)
        .json(&map)
        .send()?;
    println!("{:?}", res);
    Ok(())
}


pub trait Logger {
    // fn new<T, R>(i: T + PrintAndLog) -> TimeLogFormat + PrintAndLog;
    fn new(&self, tmp_dir: String, tmp_file_name: String) -> Self;
    fn info(&self, msg: &str) -> String;
    fn warn(&self, msg: &str)-> String;
    fn error(&self, msg: &str) -> String;
    fn debug(&self, msg: &str) -> String;
}

#[derive(Debug)]
pub struct Log {
    info: String,
    warn: String,
    error: String,
    debug: String,
    tmp_dir: String,
}


impl Log {
    pub fn new (tmp_dir: String) -> Self {
        Log {
            info: String::from("INFO"),
            warn: String::from("WARNING"),
            error: String::from("ERROR"),
            debug: String::from("DEBUG"),
            tmp_dir: tmp_dir,
        }
    }
    pub fn info(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.info, msg);
        log_msg
    }
    pub fn error(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.error, msg);
        log_msg
    }
    pub fn debug(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.debug, msg);
        log_msg
    }
    pub fn warn(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.warn, msg);
        log_msg
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timstamp() {
        if let Ok(timestamp) = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
            println!("{}", timestamp.as_secs());
        }
    }
}