use outgauge::{blocking::BlockingOutGaugeSocket, OutGauge};

fn main() -> std::io::Result<()> {
    let mut socket = BlockingOutGaugeSocket::bind("127.0.0.1:4444")?;
    let mut packet = OutGauge::default();

    loop {
        socket.receive_packet(&mut packet)?;
        println!("The motor RPM is {}!", packet.rpm);
    }

    Ok(())
}
