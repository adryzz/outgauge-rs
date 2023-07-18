use std::io::ErrorKind;
use embassy_net::udp::UdpSocket;

use crate::OutGauge;

pub struct AsyncOutGaugeSocket {
    socket: UdpSocket,
}

impl AsyncOutGaugeSocket {
    pub async fn bind(addr: &str) -> std::io::Result<Self> {
        let underlying = UdpSocket::bind(addr).await?;
        Ok(Self { socket: underlying })
    }

    pub async fn receive_packet(&mut self, packet: &mut OutGauge) -> std::io::Result<()> {
        let buf: &mut [u8; std::mem::size_of::<OutGauge>()];
        buf = unsafe { std::mem::transmute(packet) };

        let (amt, _src) = self.socket.recv_from(&mut buf[..]).await?;

        if amt != std::mem::size_of::<OutGauge>() {
            return Err(std::io::Error::new(
                ErrorKind::Interrupted,
                "Didn't receive full packet",
            ));
        }

        Ok(())
    }
}
