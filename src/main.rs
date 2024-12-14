mod sha2_definition;
mod sha256_core;

use sha2_definition::Sha256;

fn main() {
    // let message: &str = "chainedecaractere";

    let mut message = String::from("b4190e");

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
    let result = hasher.finish();
    print!("sha256: {result:x?}\n");
}
