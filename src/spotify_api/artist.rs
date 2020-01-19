extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct MinimalArtist {
    name: String,
    img_url: String
}

impl MinimalArtist {
    pub fn create(name: &str, img_url: &str) -> MinimalArtist {
        MinimalArtist {
            name: String::from(name),
            img_url: String::from(img_url)
        }
    }
}

#[wasm_bindgen]
impl MinimalArtist {
    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }

    pub fn get_img_url(&self) -> String {
        String::from(&self.img_url)
    }
}
