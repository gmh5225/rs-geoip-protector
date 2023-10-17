#[macro_use]
extern crate log;

use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;
use clap::Parser;

use tokio::net::{TcpListener, TcpStream};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Listen address and port
    #[arg(short, long, default_value_t = String::from("0.0.0.0:18830"))]
    listen: String,
    /// Remote address
    #[arg(short, long, default_value_t = String::from("127.0.0.1:8323"))]
    remote: String,
    /// Block country, allow more than one, join with comma
    #[arg(short, long, default_value_t = String::from("cn,us,jp"))]
    block: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let c_args = CliArgs::parse();

    info!("listen: {}", c_args.listen);
    info!("remote: {}", c_args.remote);
    info!("block: {}", c_args.block);
    let reader = maxminddb::Reader::open_readfile("./GeoLite2-Country.mmdb").unwrap();

    let ip: IpAddr = FromStr::from_str("113.57.107.91").unwrap();
    let country: geoip2::Country = reader.lookup(ip).unwrap();
    info!("country is {:?}", country.country.unwrap().iso_code.unwrap());


    //
    // country is "CN"

    let listener = TcpListener::bind(c_args.listen).await?;

    loop {
        let (mut socket, income_addr) = listener.accept().await?;

        info!("Accepted connection from: {:?}", income_addr);


        let income_ip = income_addr.ip().to_string();
        if income_ip.to_owned() != "127.0.0.1" && income_ip.to_owned() != "0.0.0.0" {
            let ip: IpAddr = FromStr::from_str(income_ip.as_str()).unwrap();
            let country: geoip2::Country = reader.lookup(ip).unwrap();
            info!("country is {:?}", country.country.unwrap().iso_code.unwrap());
        } else {
            info!("Localhost is just bypass");
        }
        let remote_addr = c_args.remote.to_owned();

        tokio::spawn(async move {
            let mut remote_socket = TcpStream::connect(remote_addr).await.unwrap();


            tokio::io::copy_bidirectional(&mut socket, &mut remote_socket).await.unwrap();
        });
    }
}
