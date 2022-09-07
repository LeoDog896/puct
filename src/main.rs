use anyhow::Result;
use clap::Parser;
use tokio::net::UdpSocket;

/// auto-reconnecting p2p streaming for UDP / UDP + TCP (UTP)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    IP,
    Udp {
        /// Hosting Address
        #[clap(value_parser)]
        source_address: String,

        /// Destination Address
        #[clap(value_parser)]
        destination_address: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::IP => {
            println!(
                "{}",
                public_ip::addr_v4()
                    .await
                    .map(|ip| ip.to_string())
                    .unwrap_or_else(|| "Ip address not found".to_string())
            );
        }
        Action::Udp {
            source_address,
            destination_address,
        } => {
            let sock = UdpSocket::bind(source_address).await?;

            sock.connect(destination_address).await?;

            let mut buf = [0; 1024];
            loop {
                let len = sock.recv(&mut buf).await?;
                println!("{:?} bytes received", len);

                let len = sock.send(&buf[..len]).await?;
                println!("{:?} bytes sent", len);
            }
        }
    }
    Ok(())
}
