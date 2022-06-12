use std::fs::{remove_file, File, create_dir_all, read_dir};
use std::path;
use super::utils::RANDOM_CHARS;
use random_string::generate;
use regex::Regex;
use crate::contants::NODE_LOG_HEADER;
use std::io::Read;

#[derive(Debug)]
pub struct Tmp {
    path: String,
}

impl Tmp {
    pub fn new(tmp_dir: &str) -> Tmp {
        Tmp {
            path: String::from(tmp_dir),
        }
    }
    pub fn mktmp(&self)  -> String {
    // fn mktmp(&self) -> std::io::Result<std::fs::File> {
        let tmp_path = path::Path::new(&self.path);
        if let  Err(e) = create_dir_all(tmp_path) {
            println!("{}", e);
        }
        let random_string = format!("{}{}", NODE_LOG_HEADER, self.range_file_name());
        let range_tmp_file = tmp_path.join(&random_string);
        let abs_path_tmp = format!("{}{}", self.path, &random_string);
        if ! range_tmp_file.exists() {
            if let Err(_e) = File::create(range_tmp_file) {
                println!(" create tmp file -> {} failed -> {}", abs_path_tmp, _e);
            }
        }
        abs_path_tmp
    }
    pub fn range_file_name(&self) -> String {
        generate(10, RANDOM_CHARS)
   }

   pub fn clean(&self) {
       let reg = format!("^{}.*{}.*", self.path ,NODE_LOG_HEADER);
       let tmp_file_regex: Regex = Regex::new(&reg.to_string()).unwrap();
       match read_dir(&self.path) {
           Ok(paths) => {
               for file in paths {
                   if let Ok(f) = file {
                       if tmp_file_regex.is_match(&f.path().to_string_lossy()) {
                           if let Err(e) = remove_file(&f.path()) {
                               println!("remove file failed. mgs -> {:?}", e);
                           }
                       }
                   }
               }
           }
           Err(_) => {
               println!("Couldn't read dir");
           }
       }
   }

   pub fn write(&self, tmp_file: &str, log: &str) {
       if let Err(e) = std::fs::write(tmp_file, log) {
              println!("write log failed. mgs -> {:?}", e);
       }
   }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = String::new();
        let name = "test";
        std::fs::write(name, "asdfasd\n".to_string());
        let file = File::open(name);
        let mut f = match file {
            Ok(file) => file,
            Err(e) => panic!("{}", e),
        };
        f.read_to_string(&mut data);
        assert_eq!(data, "asdfasd\n");
    }

    #[test]
    fn test_tmp_dir (){
        let tmp = Tmp::new("/tmp");
        let tmp_file = tmp.mktmp();
        Tmp::new("/tmp").clean();
    }

    #[test]
    fn test_log () {
        use crate::logs::Logger;
        let a = "test";
        let log = Logger::new();
    }
}
