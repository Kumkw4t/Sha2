pub mod sha2_lib;

use sha2_lib::sha512_definition::Sha384;

fn main() {

    // parse hexadecimal string as u8 vector
    let mut message = String::from("00");
    println!("{message}");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }
    assert_eq!(byte_vec.len(),size);

    // call Sha2 lib
    let mut hasher: Sha384 = Sha384::new();
    hasher.update(&byte_vec);
    let result = hasher.finish();

    // print result
    print!("Sha384: ");
    for elem in result.iter() {
        print!("{elem:x?}");
    }
    println!();
}
