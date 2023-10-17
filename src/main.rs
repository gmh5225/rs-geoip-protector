
use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// Listen address and port
    #[arg(short, long, default_value_t = String::from("0.0.0.0:18830"))]
    listen: String,
    /// Remote address
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3380"))]
    remote: String,
    /// Block country, allow more than one, join with comma
    #[arg(short, long, default_value_t = String::from("cn,us,jp"))]
    block: String,
}

fn main() {

    let c_args = CliArgs::parse();

        println!("listen: {}", c_args.listen);
        println!("remote: {}", c_args.remote);
        println!("block: {}", c_args.block);
    let reader = maxminddb::Reader::open_readfile("./GeoLite2-Country.mmdb").unwrap();

    let ip: IpAddr = FromStr::from_str("113.57.107.91").unwrap();
    let country: geoip2::Country = reader.lookup(ip).unwrap();
    print!("country is {:?}", country.country.unwrap().iso_code.unwrap());



    //
    // country is "CN"
}
