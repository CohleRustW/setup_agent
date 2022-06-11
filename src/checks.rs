use regex::Regex;
use std::time::Duration;
use super::functions::str_transport_to_vec;
use reqwest::{blocking, ClientBuilder};
use std::collections::HashMap;
use super::contants::*;

pub fn block_check_report_url_reachable(url: &str) ->  Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("task_id", NODE_TEST_HEADER);
    map.insert("token", NODE_TEST_HEADER);
    map.insert("logs", NODE_TEST_HEADER);
    let client = reqwest::blocking::Client::builder().timeout(CONN_TIME_OUT).build()?;
    client.post(url).json(&map).send()?;
    // TODO: 404 check
    Ok(())
}

pub fn valid_url_reg(url: &str) -> Result<(), String> {

    let url_re: Regex = Regex::new(r"^(http|https)://.*").unwrap();
    if url_re.is_match(url) {
        Ok(())
    } else {
        Err(String::from("Invalid URL"))
    }
}

pub fn valid_backend_url_reg(url: &str) -> Result<(), String> {
    let url_re: Regex = Regex::new(r"^(http|https)://.*backend/?$").unwrap();
    if url_re.is_match(url) {
        Ok(())
    } else {
        let backend_reg: Regex = Regex::new(r".*backend.*").unwrap();
        if backend_reg.is_match(url) {
            Err(String::from("URL should contain backend message"))
        } else {
            Err(String::from("Invalid URL"))
        }
    }
}

pub fn valid_ipv4_addrs (ip: &str) -> Result<(), String> {
    let ipv4_regex: Regex = Regex::new(r"[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}").unwrap();
    if ipv4_regex.is_match(ip) {
        Ok(())
    } else {
        Err(String::from("Invalid IPV4 address."))
    }

}

pub fn valid_token(token: &str) -> Result<bool, String> {
    if token.is_empty() {
        Err(String::from("Token is empty."))
    } else {
        Ok(true)
    }
}

pub fn vaild_calllback_url__token(url: &str, token: &str) -> Result<bool, Vec<String>> {
    let mut error_msgs: Vec<String> = Vec::new();
    match valid_backend_url_reg(url) {
        Ok(()) => {
            let _backend_result = true;
        },
        Err(e) => {
            error_msgs.push(e);
        }
    };
    match valid_token(token) {
        Ok(result) => {
            let _token_reuslt = result;
        },
        Err(e) => {
            error_msgs.push(e);
        }
    };
    if let (_backend_result, _token_reuslt) = (true, true) {
        Ok(true)
    } else {
        Err(error_msgs)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_url_rex () {
        let url: String = "http://www.baidu.com".to_string();
        assert_eq!(valid_url_reg(&url), Ok(()));
        let url_err: String = "http:/www.baidu.com".to_string();
        assert_eq!(valid_url_reg(&url_err), Err(String::from("Invalid URL")));
    }

    #[test]
    fn test_backend_url_rex () {
        let url: String = "http://www.baidu.com/backend".to_string();
        assert_eq!(valid_backend_url_reg(&url), Ok(()));
        let url_err: String = "http://www.baidu.com/back".to_string();
        assert_eq!(valid_backend_url_reg(&url_err), Err(String::from("Invalid URL")));
    }

    #[test]
    fn test_ipv4_address () {
        let ip: String = "10.0.1.1".to_string();
        assert_eq!(valid_ipv4_addrs(&ip), Ok(()));
        let err_ip: String = "10.01.1".to_string();
        assert_eq!(valid_ipv4_addrs(&err_ip), Err(String::from("Invalid IPV4 address")));
    }

    #[test]
    fn test_url_code () {
        // let url = "http://127.0.0.1:8000/backend/report_log/";
        let url = "http://159.75.15.90:10300/backend/report_log/";
        match block_check_report_url_reachable(&url) {
            Ok(result) => {
                println!("{:?}", result);
            },
            Err(e) => {
                let d = e.without_url();
                println!("EEEEEEEE{:?}", d);
            }
        }
    }
}
