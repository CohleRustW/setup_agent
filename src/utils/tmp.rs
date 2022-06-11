use std::fs::{remove_file, File};
use std::path;
use std::io::Read;

pub fn create_file_with_name(name: &str) -> &str {
    let file_path = path::Path::new(&name);
    if file_path.exists() {
        match remove_file(name) {
            Ok(_) => {
                println!("{} removed", name);
            }
            Err(_) => println!("Couldn't remove file {}", name),
        }
    }
    File::create(name).expect("Unable to create file");
    name
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = String::new();
        let name = "test";
        let create_file = create_file_with_name(name);
        std::fs::write(name, "asdfasd\n".to_string());
        let file = File::open(name);
        let mut f = match file {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };
        f.read_to_string(&mut data);
        assert_eq!(data, "asdfasd\n");
    }
}




