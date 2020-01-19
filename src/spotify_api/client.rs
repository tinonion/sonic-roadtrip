/**
 * Singleton object to ensure spotify API client is created only once
 */

use rspotify::spotify::client::Spotify;

use super::credentials;

pub struct SpotifyClient {
    spotify: Option<Spotify>
}

impl SpotifyClient {
    pub const fn new() -> SpotifyClient {
        SpotifyClient {
            spotify: None
        }
    }

    pub fn take_spotify(&mut self) -> &Spotify {
        if let None = self.spotify {
            self.spotify = Some(credentials::create_spotify());
        };

        self.spotify.as_ref().unwrap()
    }
}