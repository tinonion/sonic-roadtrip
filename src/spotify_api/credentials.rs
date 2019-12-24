use std::fs::File;
use std::io::{BufRead, BufReader};

const CLIENT_ID_PATH: &str = "res/client_id";
const CLIENT_SECRET_PATH: &str = "res/client_secret";

pub fn get_client_id() -> String {
    let id = read_file_line(CLIENT_ID_PATH);
    match id {
        Ok(id) => return id,
        Err(e) => panic!("expected client id from '{}', got error '{}'", CLIENT_ID_PATH, e)
    }
}

pub fn get_client_secret() -> String {
    let id = read_file_line(CLIENT_SECRET_PATH);
    match id {
        Ok(id) => return id,
        Err(e) => panic!("expected client secret from '{}', got error '{}'", CLIENT_SECRET_PATH, e)
    }
}

fn read_file_line(path: &str) -> std::io::Result<String> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    
    let mut line = String::new();
    reader.read_line(&mut line)?;

    Ok(line)
}
