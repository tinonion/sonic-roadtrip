use rspotify::spotify::senum::Country;

use super::client::SpotifyClient;
use super::artist::MinimalArtist;

static mut CLIENT: SpotifyClient = SpotifyClient::new();

pub fn search_artist(query: &str) -> Vec<MinimalArtist> {
    let spotify = unsafe { CLIENT.take_spotify() };

    let result = spotify.search_artist(query, 100, 0, Some(Country::UnitedStates));
    let result = result.unwrap();

    let first_page_artists = result.artists.items;

    assert!(first_page_artists.len() >= 1);

    first_page_artists.iter()
        .map(|full_artist| MinimalArtist::create(
            &full_artist.name, 
            &full_artist.images[0].url))
        .collect()
}