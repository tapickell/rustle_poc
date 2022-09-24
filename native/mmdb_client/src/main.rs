use maxminddb::geoip2;
use std::net::IpAddr;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let reader = maxminddb::Reader::open_readfile(
        args.next()
            .ok_or("First argument must be the path to the IP database")?,
    ) // IoError("No such file or directory (os error 2)")
    .unwrap();
    let ip: IpAddr = args
        .next()
        .ok_or("Second argument must be the IP address, like 128.101.101.101")?
        .parse()
        .unwrap(); // AddrParseError(Ip)

    let city: geoip2::City = reader.lookup(ip).unwrap();
    match city.country {
        Some(geoip2::country::Country {
            geoname_id: _,
            is_in_european_union: _,
            names: _,
            iso_code: Some(country_code),
        }) => println!("Conutry Code: {}", country_code),
        _ => {
            println!("{:#?}", city);
            println!("Country code not found in city data")
        }
    }
    Ok(())
}
