use core::net::{IpAddr, Ipv4Addr, SocketAddr};
use core::time::Duration;

use edge_captive::io::run;

use log::*;

fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let stack = edge_nal_std::Stack::new();

    let mut tx_buf = [0; 1500];
    let mut rx_buf = [0; 1500];

    info!("Running Captive Portal DNS on UDP port 8853...");

    futures_lite::future::block_on(run(
        // Can't use DEFAULT_SOCKET because it uses DNS port 53 which needs root
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8853),
        &mut tx_buf,
        &mut rx_buf,
        Ipv4Addr::new(192, 168, 0, 1),
        Duration::from_secs(60),
    ))
    .unwrap();
}
