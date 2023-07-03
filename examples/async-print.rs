use outgauge::{nonblocking::AsyncOutGaugeSocket, OutGauge};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut socket = AsyncOutGaugeSocket::bind("127.0.0.1:4444").await?;
    let mut packet = OutGauge::default();

    loop {
        socket.receive_packet(&mut packet).await?;
        println!("The motor RPM is {}!", packet.rpm);
    }
}
