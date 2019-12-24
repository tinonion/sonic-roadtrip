use std::fs::File;
use std::io::{BufRead, BufReader};

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;

const CLIENT_ID_PATH: &str = "res/client_id";
const CLIENT_SECRET_PATH: &str = "res/client_secret";

pub fn get_spotify() -> Spotify {
    let client_id = get_client_id();
    let client_secret = get_client_secret();

    let client_credentials = SpotifyClientCredentials::default()
        .client_id(&client_id)
        .client_secret(&client_secret)
        .build();

    Spotify::default()
        .client_credentials_manager(client_credentials)
        .build()
}

fn get_client_id() -> String {
    let id = read_file_line(CLIENT_ID_PATH);
    match id {
        Ok(id) => return id,
        Err(e) => panic!("expected client id from '{}', got error '{}'", CLIENT_ID_PATH, e)
    }
}

fn get_client_secret() -> String {
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
