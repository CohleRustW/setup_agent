mod checks;
mod logs;

use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(short='I', long, validator= checks::valid_ipv4_addrs)]
    lan_eth_ip: String,

    #[clap(short='i', long)]
    cloud_id: usize,

    #[clap(short='l', long, validator = checks::valid_url_reg)]
    download_url: String,

    #[clap(short='s', long)]
    task_id: String,

    #[clap(short='u', long, takes_value = false, forbid_empty_values = false, required = false, default_missing_value = "true", default_value= "false")]
    upgrade: String,

    #[clap(short='c', long)]
    token: String,

    #[clap(short='r', long, validator = checks::valid_backend_url_reg)]
    callback_url: String,

    #[clap(short='x', long, takes_value = false, forbid_empty_values = false, required = false, default_value= "false")]
    http_proxy: String,

    #[clap(short = 'p' , long)]
    agent_setup_path: String,

    #[clap(short = 'e', long)]
    bk_file_server_ips: String,

    #[clap(short = 'a', long)]
    data_server_ips: String,

    #[clap(short = 'k', long)]
    task_server_ips: String,

    #[clap(short = 'N', long)]
    upstream_tyep: String,

    #[clap(short='v', long, takes_value = false, forbid_empty_values = false, required = false, default_value= "false")]
    vars_list: String,

    #[clap(short='o', long, takes_value = false, forbid_empty_values = false, required = false, default_missing_value = "true", default_value= "false")]
    overide: String,

    #[clap(short = 'T', long)]
    tmp_dir: String,

    #[clap(short='R', long, takes_value = false, forbid_empty_values = false, required = false, default_missing_value = "true", default_value= "false")]
    remove: String,

    #[clap(short='D', long, takes_value = false, forbid_empty_values = false, required = false, default_missing_value = "true", default_value= "false")]
    debug: String,

    #[clap(short = 'O', long)]
    io_port: String,

    #[clap(short = 'E', long)]
    file_svr_port: usize,

    #[clap(short = 'A', long)]
    data_port: usize,

    #[clap(short = 'V', long)]
    btsvr_thrift_port : String,

    #[clap(short = 'B', long)]
    bt_port: usize,

    #[clap(short = 'S', long)]
    bt_port_start: usize,

    #[clap(short = 'Z', long)]
    bt_port_end: usize,

    #[clap(short = 'K', long)]
    tracker_port: usize,
}


fn main() {
    let args = Args::parse();
    println!("{:?}", args);
    println!("Hello {}!", args.upgrade)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_args () {
        todo!();
    }
}