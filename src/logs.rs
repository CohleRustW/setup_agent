use super::functions::local_time_format;
use std::cmp::min;
use reqwest::{blocking, StatusCode, Response};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::contants::{INFO, ERROR, WARN, DEBUG};
use crate::utils::tmp::Tmp;
use crate::Args;


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

pub struct Print {
    tmp: Tmp,
}

pub struct Report {
    tmp: Tmp,
    args: Args,
}

pub trait Logger {
    // fn new<T, R>(i: T + PrintAndLog) -> TimeLogFormat + PrintAndLog;
    fn new() -> Print;
    fn info(&self, msg: &str) -> String;
    fn warn(&self, msg: &str)-> String;
    fn error(&self, msg: &str) -> String;
    fn debug(&self, msg: &str) -> String;
}


impl Logger for Print {

    fn new() -> Self {
        Print {
            tmp: Tmp::new(),
        }
    }
    fn info(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), INFO, msg);
        &self.tmp.write(&log_msg);
        println!("{}", log_msg);
        log_msg
    }
    fn error(&self, msg: &str) -> String {
        let _log_msg = format!("{} {} {}", local_time_format(), ERROR, msg);
        &self.tmp.write(&_log_msg);
        println!("{}", _log_msg);
        _log_msg
    }
    fn debug(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), DEBUG, msg);
        &self.tmp.write(&log_msg);
        log_msg
    }
    fn warn(&self, msg: &str) -> String {
        let log_msg = format!("{} {} {}", local_time_format(), WARN, msg);
        &self.tmp.write(&log_msg);
        println!("{}", log_msg);
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
    #[test]
    #[warn(deprecated)]
    fn test_log_info () {
        use crate::contants::NODE_LOG_HEADER;
        use std::time::Duration;
        let sleep_time: Duration = Duration::from_secs(1);
        use regex::Regex;
        use std::fs::read_dir;
        let print = Print::new();
        let t = Tmp::new();
        t.clean();
        use std::thread;
        thread::sleep(sleep_time);
        print.info("test");
        thread::sleep(sleep_time);
        print.info("test");
        thread::sleep(sleep_time);
        print.info("test");
        print.info("test");
        let tmp_file = "/tmp";
        let reg = format!("^{}.*{}.*", "/tmp", NODE_LOG_HEADER);
        let tmp_file_regex: Regex = Regex::new(&reg.to_string()).unwrap();
        let mut macth_file_vec = Vec::new();
        match read_dir(&tmp_file) {
            Ok(paths) => {
                for file in paths {
                    if let Ok(f) = file {
                        if tmp_file_regex.is_match(&f.path().to_string_lossy()) {
                            macth_file_vec.push(f.path().to_string_lossy().to_string());
                        }
                    }
                }
            }
            Err(_) => {
                println!("Couldn't read dir");
            }
        }
        assert_eq!(macth_file_vec.len(), 1);
    }
}