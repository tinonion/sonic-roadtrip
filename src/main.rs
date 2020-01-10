mod spotify_api;

use rspotify::spotify::senum::Country;

use spotify_api::credentials;

fn main() {
    let spotify = credentials::get_spotify();

    let q = "White Buffalo";
    let result = spotify.search_artist(q, 10, 0, Some(Country::UnitedStates));

    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let artist = spotify.artist(birdy_uri);

    println!("artist: {:?}", result);
}
