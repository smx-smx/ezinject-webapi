use ezinject_webapi::server;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

fn main() {
    println!("[*] Starting WebAPI host");

    server::start_server_task(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        8000
    ).expect("Failed to start HTTP Server");
}
