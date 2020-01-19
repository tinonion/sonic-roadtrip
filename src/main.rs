mod spotify_api;

use spotify_api::artist_search;

fn main() {
    let q = "Led Zeppelin";
    let result = artist_search::search_artist(q);

    println!("artist: {:#?}", result);
}
