mod spotify_api;

use spotify_api::credentials;

fn main() {
    println!("client id: {}", credentials::get_client_id());
    println!("client secret: {}", credentials::get_client_secret());
}
