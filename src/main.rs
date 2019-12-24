mod spotify_api;

use spotify_api::credentials;

fn main() {
    let spotify = credentials::get_spotify();

    let uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP"; 
    let artist = spotify.artist(uri);

    println!("artist: {:?}", artist);
}
