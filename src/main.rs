mod sha256_definition;
mod sha512_definition;
mod sha256_core;
mod sha512_core;

use sha512_definition::Sha384;

fn main() {
    let mut message = String::from("00");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }
    assert_eq!(byte_vec.len(),size);
    println!("{byte_vec:?}");

    let mut hasher: Sha384 = Sha384::new();
    hasher.update(&byte_vec);
    let result = hasher.finish();
    print!("Sha384: ");
    for elem in result.iter() {
        print!("{elem:x?}");
    }
    println!();
}
