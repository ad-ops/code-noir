use photon_rs::monochrome;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greyscale_image(data: Vec<u8>) -> Vec<u8>{
    let mut img: photon_rs::PhotonImage = photon_rs::PhotonImage::new_from_byteslice(data.to_vec());
    monochrome::grayscale(&mut img);

    return img.get_bytes();
}