use regex::Regex;
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
        Err(String::from("Invalid IPV4 address"))
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
}