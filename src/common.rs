use std::fs::{read_to_string, File};
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

pub fn get_test_input() -> std::io::Result<String> {
    read_to_string("test.txt")
}

pub fn get_input(day: i8) -> std::io::Result<String> {
    dotenv::dotenv().expect(".env");
    let session = std::env::var("session").expect("env session");
    let filename = format!("{}.txt", day);
    let path = Path::new(&filename);
    if path.exists() {
        return read_to_string(path);
    }
    let url = format!("https://adventofcode.com/2021/day/{}", day);
    let s = ureq::get(&format!("{}/input", url))
        .set("referer", &url)
        .set("cookie", &format!("session={}", session))
        .call()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        .into_string()?;
    File::create(path)?.write_all(s.as_bytes())?;
    Ok(s)
}
