
use maxminddb::geoip2;
use std::net::IpAddr;
use std::str::FromStr;
fn main() {

    let reader = maxminddb::Reader::open_readfile("./GeoLite2-Country.mmdb").unwrap();

    let ip: IpAddr = FromStr::from_str("113.57.107.91").unwrap();
    let country: geoip2::Country = reader.lookup(ip).unwrap();
    print!("country is {:?}", country.country.unwrap().iso_code.unwrap());



    //
    // country is "CN"
}
