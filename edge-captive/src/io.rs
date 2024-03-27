use core::fmt;
use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use core::time::Duration;

use edge_nal::{UdpBind, UdpReceive, UdpSend};

use log::*;

use super::*;

pub const DEFAULT_SOCKET: SocketAddr = SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), PORT);

const PORT: u16 = 53;

#[derive(Debug)]
pub enum DnsIoError {
    DnsError,
    IoError,
}

#[cfg(feature = "std")]
pub async fn run(
    local_addr: SocketAddr,
    tx_buf: &mut [u8],
    rx_buf: &mut [u8],
    ip: Ipv4Addr,
    ttl: Duration,
) -> Result<(), DnsIoError> {
    use std::net::UdpSocket;

    let mut udp = UdpSocket::bind(local_addr).map_err(|_| DnsIoError::IoError)?;

    loop {
        debug!("Waiting for data");

        let (len, remote) = udp.recv_from(rx_buf).map_err(|_| DnsIoError::IoError)?;

        let request = &rx_buf[..len];

        debug!("Received {} bytes from {remote}", request.len());

        let len = match crate::reply(request, &ip.octets(), ttl, tx_buf) {
            Ok(len) => len,
            Err(err) => match err {
                DnsError::InvalidMessage => {
                    warn!("Got invalid message from {remote}, skipping");
                    continue;
                }
                other => Err(DnsIoError::DnsError)?,
            },
        };

        udp.send_to(&tx_buf[..len], remote)
            .map_err(|_| DnsIoError::IoError)?;

        debug!("Sent {len} bytes to {remote}");
    }
}
