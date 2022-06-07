
pub fn ipv4_transport_to_vec (ip: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
	let ip_str_vec: Vec<&str> = ip.split(".").collect();
	let mut ip_u8_vec: Vec<u8> = Vec::new();
	for str_num in ip_str_vec {
        let num: u8 = str_num.parse::<u8>()?;
        ip_u8_vec.push(num);
	}
	Ok(ip_u8_vec)
}

pub fn str_transport_to_bool(s: &str) -> Result<bool, std::num::ParseIntError> {
	let result: bool = s.parse::<bool>()?;
	result
}