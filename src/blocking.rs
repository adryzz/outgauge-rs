use std::{net::UdpSocket, io::ErrorKind};

use crate::OutGauge;

pub struct BlockingOutGaugeSocket {
    socket: UdpSocket
}

impl BlockingOutGaugeSocket {
    pub fn bind(addr: &str) -> std::io::Result<Self> {
        let underlying = UdpSocket::bind(addr)?;
        Ok(Self {
            socket: underlying
        })
    }

    pub fn receive_packet(&mut self, packet: &mut OutGauge) -> std::io::Result<()> {
        let buf: &mut [u8; std::mem::size_of::<OutGauge>()];
        buf = unsafe { std::mem::transmute(packet) };

        let (amt, _src) = self.socket.recv_from(&mut buf[..])?;

        if amt != std::mem::size_of::<OutGauge>() {
            return Err(std::io::Error::new(ErrorKind::Interrupted, "Didn't receive full packet"));
        }

        Ok(())
    }
}