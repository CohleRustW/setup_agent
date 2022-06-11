use clap::Parser;
use clap::ArgMatches;
use crate::Args;
use crate::functions;
use crate::checks::{block_check_report_url_reachable, vaild_calllback_url__token};
use std::time::Duration;

pub static REPORT_LOG_URL: &'static str = "report_log";
pub static CONN_TIME_OUT: Duration = Duration::from_secs(5);
pub static NODE_TEST_HEADER: &'static str = "NodeManScriptTest";

pub struct Constants {
	pub conn_timeout_time: usize,
    pub report_log_url:  &'static str,
}

impl Constants {
    pub fn new (&self) -> Self {
        Constants {
            conn_timeout_time: CONN_TIME_OUT,
            report_log_url: REPORT_LOG_URL,
        }
    }
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct RuntimeEnv<'a> {
    pub lan_eth_ip: &'a str,
    pub cloud_id: usize,
    pub download_url: &'a str,
    pub task_id: &'a str,
    pub upgrade: bool,
    pub token: &'a str,
    pub callback_url:&'a str,
    pub http_proxy: &'a str,
    pub agent_setup_path: &'a str,
    pub bk_file_server_ips: Vec<&'a str>,
    pub data_server_ips: Vec<&'a str>,
    pub task_server_ips: Vec<&'a str>,
    pub upstream_type: &'a str,
    pub vars_list: &'a str,
    pub overide: bool,
    pub tmp_dir: &'a str,
    pub remove: bool,
    pub debug: bool,
    pub io_port: &'a str,
    pub file_svr_port: usize,
    pub data_port: usize,
    pub btsvr_thrift_port: usize,
    pub bt_port: usize,
    pub bt_port_start: usize,
    pub bt_port_end: usize,
    pub tracker_port: usize,
}

#[derive(Debug)]
pub struct Config<'b>{
    pub runtime_env: RuntimeEnv<'b>,
    pub check_error_msg: Vec<String>,
    pub valid_backenc_url: bool
}


pub fn load_contants<'b> (args: &'b Args)  -> Result<Config<'b>, Box<dyn std::error::Error>> {

        let mut args_err_logs: Vec<String> = Vec::new();
        let mut upgrade: bool = false;

        // match callbackupurl, if error , exit
        if let Err(result) = vaild_calllback_url__token(&args.callback_url, &args.token) {
            println!("callback_url or token is invalid, please check it\n");
            println!("Error mgs -> {:?}\n", result);
            std::process::exit(1);
        }

        if let Ok(result) = functions::str_transport_to_bool(&args.upgrade) {
            upgrade = result;
        } else {
            args_err_logs.push("Args upgrade param is not bool.".to_string());
        }

        if let Ok(result) = functions::str_transport_to_vec(&args.bk_file_server_ips) {
            if result.len() == 0 {
                args_err_logs.push("Args bk_file_server_ips param is empty.".to_string());
            }
        } else {
            args_err_logs.push("Args bk_file_server_ips param is not string.".to_string());
        }

        if let Ok(result) = functions::str_transport_to_vec(&args.bk_file_server_ips) {
            if result.len() == 0 {
                args_err_logs.push(format!("Args bk_file_server_ips param is empty."));
            } else {
                args_err_logs.push(format!("Args bk_file_server_ips param is not string"));
            }
        }
        if let Ok(result) = functions::str_transport_to_vec(&args.data_server_ips) {
            if result.len() == 0 {
                args_err_logs.push(format!("Args data_server_ips param is empty."));
            } else {
                args_err_logs.push(format!("Args data_server_ips param is not string"));
            }
        }
        if let Ok(result) = functions::str_transport_to_vec(&args.task_server_ips) {
            if result.len() == 0 {
                args_err_logs.push(format!("Args task_server_ips param is empty."));
            } else {
                args_err_logs.push(format!("Args task_server_ips param is not string"));
            }
        }

        let report_log_url: String = format!("{}/{}", &args.callback_url, REPORT_LOG_URL);
        let mut _valid_backenc_url: bool = true;
        if let Err(err) = block_check_report_url_reachable(&report_log_url) {
            args_err_logs.push(format!("report_log_url is invalid, please check it\n"));
            args_err_logs.push(format!("Error mgs -> {:?}\n", err));
            _valid_backenc_url = false;
        }

        let remove: bool = functions::str_transport_to_bool(&args.remove)?;
        let overide: bool = functions::str_transport_to_bool(&args.overide)?;

        let runtime_env = RuntimeEnv {
            lan_eth_ip: &args.lan_eth_ip,
            cloud_id: args.cloud_id,
            download_url: &args.download_url,
            task_id: &args.task_id,
            upgrade: upgrade,
            token: &args.token,
            callback_url: &args.callback_url,
            http_proxy: &args.http_proxy,
            agent_setup_path: &args.agent_setup_path,
            bk_file_server_ips: functions::str_transport_to_vec(&args.bk_file_server_ips)?,
            data_server_ips: functions::str_transport_to_vec(&args.data_server_ips)?,
            task_server_ips: functions::str_transport_to_vec(&args.task_server_ips)?,
            upstream_type: &args.upstream_type,
            vars_list: &args.vars_list,
            overide: overide,
            tmp_dir: &args.tmp_dir,
            remove: remove,
            debug: functions::str_transport_to_bool(&args.debug)?,
            io_port: &args.io_port,
            file_svr_port: args.file_svr_port,
            data_port: args.data_port,
            btsvr_thrift_port: args.btsvr_thrift_port,
            bt_port: args.bt_port,
            bt_port_start: args.bt_port_start,
            bt_port_end: args.bt_port_end,
            tracker_port: args.tracker_port,
        };
        Ok(Config {
            runtime_env: runtime_env,
            check_error_msg: args_err_logs,
            valid_backenc_url: _valid_backenc_url,
        })
}

