use clap::Parser;
use clap::ArgMatches;
use crate::Args;
use crate::functions;
pub struct Constants {
	pub conn_timeout_time: u64,
}

pub static CONSTANTS: Constants = Constants {
	conn_timeout_time: 5
};


pub struct RuntimeEnv<'a> {
    lan_eth_ip: &'a str,
    cloud_id: usize,
    download_url: &'a str,
    task_id: &'a str,
    upgrade: bool,
    token: &'a str,
    callback_url:&'a str, 
    http_proxy: &'a str,
    agent_setup_path: &'a str,
    bk_file_server_ips: Vec<&'a str>,
    data_server_ips: Vec<&'a str>,
    task_server_ips: Vec<&'a str>,
    upstream_type: &'a str,
    vars_list: &'a str,
    overide: bool,
    tmp_dir: &'a str,
    remove: bool,
    debug: bool,
    io_port: &'a str,
    file_svr_port: usize,
    data_port: usize,
    btsvr_thrift_port: usize,
    bt_port: usize,
    bt_port_start: usize,
    bt_port_end: usize,
    tracker_port: usize,
}

pub fn load_contants<'b> (args: &'b Args)  -> Result<RuntimeEnv<'b>, Box<dyn std::error::Error>> {

        let mut args_err_logs: Vec<String> = Vec::new();
        let mut upgrade: bool = false;
        if let Ok(upgarde) = functions::str_transport_to_bool(&args.upgrade) {
            upgrade = upgarde;
        } else {
            args_err_logs.push("Args upgrade param is not bool.".to_string());
        }

        let bk_file_server_ips: Vec<&str> = functions::str_transport_to_vec(&args.bk_file_server_ips)?;
        let data_server_ips: Vec<&str> = functions::str_transport_to_vec(&args.data_server_ips)?;
        let task_server_ips: Vec<&str> = functions::str_transport_to_vec(&args.task_server_ips)?;

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
            bk_file_server_ips: bk_file_server_ips,
            data_server_ips: data_server_ips,
            task_server_ips: task_server_ips,
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
        Ok(runtime_env)
}
