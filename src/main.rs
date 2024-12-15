mod sha256_definition;
mod sha512_definition;
mod sha256_core;
mod sha512_core;

use sha256_definition::Sha224;
use sha512_definition::Sha512;

fn main() {
    let mut message = String::from("fd2203e467574e834ab07c9097ae164532f24be1eb5d88f1af7748ceff0d2c67a21f4e4097f9d3bb4e9fbf97186e0db6db0100230a52b453d421f8ab9c9a6043aa3295ea20d2f06a2f37470d8a99075f1b8a8336f6228cf08b5942fc1fb4299c7d2480e8e82bce175540bdfad7752bc95b577f229515394f3ae5cec870a4b2f8");

    let size = message.len()/2;
    let mut byte_vec: Vec<u8> = Vec::new();

    while !message.is_empty() {
        let cur_byte: String = message.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }
    assert_eq!(byte_vec.len(),size);

    // let mut hasher: Sha224 = Sha224::new();
    // hasher.update(&byte_vec);
    // let result = hasher.finish();
    // print!("Sha224: ");
    // for elem in result.iter() {
    //     print!("{elem:x?}");
    // }
    // println!();

    let mut hasher: Sha512 = Sha512::new();
    hasher.update(&byte_vec);
    let result = hasher.finish();
    print!("Sha512: ");
    for elem in result.iter() {
        print!("{elem:x?}");
    }
    println!();
}
