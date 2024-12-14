mod sha2_definition;
mod sha256_core;

use sha2_definition::Sha256;

fn main() {
    let mut message = String::from("451101250ec6f26652249d59dc974b7361d571a8101cdfd36aba3b5854d3ae086b5fdd4597721b66e3c0dc5d8c606d9657d0e323283a5217d1f53f2f284f57b85c8a61ac8924711f895c5ed90ef17745ed2d728abd22a5f7a13479a462d71b56c19a74a40b655c58edfe0a188ad2cf46cbf30524f65d423c837dd1ff2bf462ac4198007345bb44dbb7b1c861298cdf61982a833afc728fae1eda2f87aa2c9480858bec");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }
    assert_eq!(byte_vec.len(),size);

    let mut hasher: Sha256 = Sha256::new();
    hasher.update(&byte_vec);
    let result = hasher.finish();
    print!("Sha256: ");
    for elem in result.iter() {
        print!("{elem:x?}");
    }
    println!();
}
