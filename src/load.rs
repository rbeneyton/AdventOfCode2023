use crate::Day;

use std::io::{Error, ErrorKind};

use curl::easy::Easy;
use std::fs::{create_dir, write};

pub fn get_data_server(day: Day, session: &String) -> Result<(), Error> {
    let mut res = Vec::new();
    let mut easy = Easy::new();

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    easy.url(&url).unwrap();
    let cookie = format!("session={}", session);
    easy.cookie(&cookie).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            res.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let code = easy.response_code().unwrap();
    if code != 200 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("invalid HTTP response: {}", code),
        ));
    }

    let data = String::from_utf8_lossy(&res).to_string();

    let dir = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
    let _ = create_dir(&dir);

    let file = format!("{}/{:02}.input", dir, day);
    write(&file, &data)?;
    Ok(())
}
