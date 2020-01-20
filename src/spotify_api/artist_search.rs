use rspotify::spotify::senum::Country;

use super::client::SpotifyClient;
use super::artist::MinimalArtist;

static mut CLIENT: SpotifyClient = SpotifyClient::new();

pub fn search_artist(query: &str) -> Vec<MinimalArtist> {
    let spotify = unsafe { CLIENT.take_spotify() };

    let result = spotify.search_artist(query, 10, 0, Some(Country::UnitedStates));

    let first_page_artists = match result {
        Ok(search) => search.artists.items,
        Err(e) => panic!("error found {:?} for query {}", e, query)
    };

    first_page_artists.iter()
        .map(|full_artist| MinimalArtist::create(
            &full_artist.name, 
            if full_artist.images.len() >= 1 {
                &full_artist.images[0].url
            } else {
                "IMAGE N/A"
            }))
        .collect()
}