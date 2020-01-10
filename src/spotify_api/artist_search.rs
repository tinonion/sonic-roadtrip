use rspotify::client::Spotify;
use rspotify::spotify::senum::Country;

pub search_artist(spotify: Spotify, query: &str) -> Vec<String> {
    let result = spotify.search_artist(query, 100, 0, Some(Country::UnitedStates));


}