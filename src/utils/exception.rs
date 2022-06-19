use crate::functions::local_time_format;

pub trait NodeError {
    fn create_tmp_error() ;
}

pub struct Exit;

impl NodeError for Exit {
    fn create_tmp_error () {
        let log = format!("{} {}", local_time_format(), "创建 Tmp file 失败".to_string());
        println!("{}", log);
        std::process::exit(1)
    }

}