mod sha256_definition;
mod sha256_core;

use sha256_definition::Sha224;

fn main() {
    let mut message = String::from("5f77b3664823c33e");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }
    assert_eq!(byte_vec.len(),size);

    let mut hasher: Sha224 = Sha224::new();
    hasher.update(&byte_vec);
    let result = hasher.finish();
    print!("Sha224: ");
    for elem in result.iter() {
        print!("{elem:x?}");
    }
    println!();

}
