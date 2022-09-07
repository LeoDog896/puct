use clap::Parser;
use tokio::net::UdpSocket;
use anyhow::Result;

/// PUCT
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// Hosting Address
   #[clap(value_parser)]
   source_address: String,

    /// Destination Address
    #[clap(value_parser)]
    destination_address: String
}

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    let sock = UdpSocket::bind(args.source_address).await?;

    sock.connect(args.destination_address).await?;

    let mut buf = [0; 1024];
    loop {
        let len = sock.recv(&mut buf).await?;
        println!("{:?} bytes received", len);

        let len = sock.send(&buf[..len]).await?;
        println!("{:?} bytes sent", len);
    }
}