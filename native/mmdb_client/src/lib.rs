use maxminddb::geoip2;
use rustler::Atom;
//use std::net::IpAddr;

mod atoms {
    rustler::atoms! {
        country_code_not_found_in_city_data,
        address_not_found_in_database,
        database_not_found,
        ip_address_invalid,
    }
}

// Flying V Formation :joy:
// ![](https://s3-us-west-2.amazonaws.com/courses-images/wp-content/uploads/sites/1865/2017/05/04203601/Screen-Shot-2016-06-21-at-10.52.04-AM-300x186.png)
//
#[rustler::nif]
fn country_code_for_ip_address(ip_address: String, db_path: String) -> Result<String, Atom> {
    match maxminddb::Reader::open_readfile(db_path) {
        Ok(reader) => {
            match ip_address.parse() {
                Ok(ip) => {
                    match reader.lookup::<geoip2::City>(ip) {
                        Ok(city) => match city.country {
                            Some(geoip2::country::Country {
                                geoname_id: _,
                                is_in_european_union: _,
                                names: _,
                                iso_code: Some(country_code),
                            }) => Ok(country_code.to_string()),
                            _ => Err(atoms::country_code_not_found_in_city_data()),
                        },
                        _ => Err(atoms::address_not_found_in_database()), // :shrug: what errors can happen here??
                    }
                }
                _ => Err(atoms::ip_address_invalid()), // AddrParseError(Ip)
            }
        }
        _ => Err(atoms::database_not_found()), // IoError("No such file or directory (os error 2)")
    }
}

rustler::init!("Elixir.RustlePoc.Native", [country_code_for_ip_address]);
