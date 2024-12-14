mod sha2_definition;
mod sha256_core;

use sha2_definition::Sha256;

fn main() {
    // let message: &str = "chainedecaractere";

    let mut message = String::from("120b80b44a2435d4783766416ede1784e00d3467c7ed1b6304e75b5846febb812d637fa6b043e693bce4f7ca1df07d75a211519e7cbdd7a2e854cbe30e683ebe285bdbbde6306f0d119ba932705a1ef9");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }

    assert_eq!(byte_vec.len(),size);
    println!("{byte_vec:?}");

    let mut hasher: Sha256 = Sha256::new();
    hasher.update(byte_vec);
}
