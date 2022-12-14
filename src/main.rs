use anyhow::Result;
use clap::Parser;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::UdpSocket;

/// Auto-reconnecting p2p streaming for UDP / UDP + TCP (UTP)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Prints the current public IP of the runner
    IP,
    /// Runs puct with the UDP protocol
    Udp {
        /// Destination Address
        #[clap(value_parser)]
        destination_address: String,

        /// Hosting Address. Default to your public IP from `puct ip`.
        #[clap(value_parser, long, short)]
        source_address: Option<String>,
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
                    .map_or_else(|| "Ip address not found".to_string(), |ip| ip.to_string())
            );
        }
        Action::Udp {
            source_address,
            destination_address,
        } => {
            let public_addr = public_ip::addr_v4()
                .await
                .map(|addr| SocketAddrV4::new(addr, 5713));
            let parsed_address = source_address
                .clone()
                .map(|str| {
                    str.parse::<SocketAddrV4>().or_else(|_| {
                        source_address
                            .clone()
                            .unwrap()
                            .parse::<Ipv4Addr>()
                            .map(|addr| SocketAddrV4::new(addr, 5713))
                    })
                })
                .transpose()?
                .or(public_addr)
                .expect("Could not find a public IP address");
            let sock = UdpSocket::bind(parsed_address).await?;

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
