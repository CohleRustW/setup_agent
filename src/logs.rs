use super::functions::local_time_format;
use std::cmp::min;
use reqwest::{blocking, StatusCode, Response};
use std::collections::HashMap;


fn get_report_step_status(url: &str, task_id: &str, token: &str, logs: &str) ->  Result<(), Box<dyn std::error::Error>> {
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

pub struct Log {
    info: String,
    warn: String,
    error: String,
    debug: String,
    tmp_dir: String,
    tmp_file_name: String,
    report: bool,
}


impl Logger for Log {
    fn new (&self, tmp_dir: String, tmp_file_name: String) -> Self {
        Log {
            info: String::from("INFO"),
            warn: String::from("WARNING"),
            error: String::from("ERROR"),
            debug: String::from("DEBUG"),
            tmp_dir: tmp_dir,
            tmp_file_name: tmp_file_name,
            report: false,
        }
    }
    fn info(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.info, msg);
        log_msg
    }
    fn error(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.error, msg);
        log_msg
    }
    fn debug(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.debug, msg);
        log_msg
    }
    fn warn(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), &self.warn, msg);
        log_msg
    }
}