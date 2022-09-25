use maxminddb::geoip2;
use rustler::Atom;
use std::net::IpAddr;

mod atoms {
    rustler::atoms! {
        country_code_not_found_in_city_data,
        address_not_found_in_database,
        database_not_found,
        ip_address_invalid,
    }
}

#[rustler::nif]
fn country_code_for_ip_address(ip_address: String, db_path: String) -> Result<String, Atom> {
    let reader =
        maxminddb::Reader::open_readfile(db_path).map_err(|_| atoms::database_not_found())?;
    let ip: IpAddr = ip_address
        .parse()
        .map_err(|_| atoms::ip_address_invalid())?;

    let city: geoip2::City = reader
        .lookup(ip)
        .map_err(|_| atoms::address_not_found_in_database())?;

    match city.country {
        Some(geoip2::country::Country {
            iso_code: Some(country_code),
            ..
        }) => Ok(country_code.to_string()),
        _ => Err(atoms::country_code_not_found_in_city_data()),
    }
}

rustler::init!("Elixir.RustlePoc.Native", [country_code_for_ip_address]);
