use clap::Parser;
use clap::ArgMatches;
use crate::Args;
use crate::functions::{str_transport_to_bool, ipv4_transport_to_vec};
pub struct Constants {
	pub conn_timeout_time: u64,
}

pub static CONSTANTS: Constants = Constants {
	conn_timeout_time: 5
};


pub struct RuntimeEnv {
    lan_eth_ip: String,
    cloud_id: usize,
    download_url: String,
    task_id: String,
    upgrade: bool,
    token: String,
    callback_url: String,
    http_proxy: String,
    agent_setup_path: String,
    bk_file_server_ips: Vec<String>,
    data_server_ips: Vec<String>,
    task_server_ips: Vec<String>,
    upstream_tyep: String,
    vars_list: String,
    overide: bool,
    tmp_dir: String,
    remove: bool,
    debug: bool,
    io_port: String,
    file_svr_port: usize,
    data_port: usize,
    btsvr_thrift_port: usize,
    bt_port: usize,
    bt_port_start: usize,
    bt_port_end: usize,
    tracker_port: usize,
}

pub fn load_contants (args: Args)  -> RuntimeEnv {

        let args_err_logs: Vec<String> = Vec::new();
        let mut upgrade: bool = false;
        if let Ok(upgarde) = str_transport_to_bool(&args.upgrade) {
            upgrade = upgarde;
        } else {
            args_err_logs.push("Args upgrade param is not bool.".to_string());
        }

    let runtime_env = RuntimeEnv {
        lan_eth_ip: args.lan_eth_ip,
        cloud_id: args.cloud_id,
        download_url: args.download_url,
        task_id: args.task_id,
        upgrade: upgrade,
        token: args.token,
        callback_url: args.callback_url,
        http_proxy: args.http_proxy,
        agent_setup_path: args.agent_setup_path,
        bk_file_server_ips: args.bk_file_server_ips,
        data_server_ips: args.data_server_ips,
        task_server_ips: args.task_server_ips,
        upstream_tyep: args.upstream_tyep,
        vars_list: args.vars_list,
        overide: args.overide,
        tmp_dir: args.tmp_dir,
    };
    runtime_env
}