pub mod sha2_lib;

use sha2_lib::Hasher;
use sha2_lib::sha512_definition::{Sha384, Sha512, Sha512_224, Sha512_256};
use sha2_lib::sha256_definition::{Sha224, Sha256};

enum HasherEnum {
    Sha224(Sha224),
    Sha256(Sha256),
    Sha384(Sha384),
    Sha512(Sha512),
    Sha512_224(Sha512_224),
    Sha512_256(Sha512_256),
}

impl HasherEnum {
    fn update(&mut self,input: &Vec<u8>) {
        match self {
            HasherEnum::Sha224(hasher)          => hasher.update(input),
            HasherEnum::Sha256(hasher)          => hasher.update(input),
            HasherEnum::Sha384(hasher)          => hasher.update(input),
            HasherEnum::Sha512(hasher)          => hasher.update(input),
            HasherEnum::Sha512_224(hasher)  => hasher.update(input),
            HasherEnum::Sha512_256(hasher)  => hasher.update(input),
        }
    }

    fn finish(self) -> Vec<u8> {
        match self {
            HasherEnum::Sha224(hasher)          => hasher.finish(),
            HasherEnum::Sha256(hasher)          => hasher.finish(),
            HasherEnum::Sha384(hasher)          => hasher.finish(),
            HasherEnum::Sha512(hasher)          => hasher.finish(),
            HasherEnum::Sha512_224(hasher)  => hasher.finish(),
            HasherEnum::Sha512_256(hasher)  => hasher.finish(),
        }
    }
}

fn calculate_hash(mut input: String, algo: &str) -> String {


    // Convert string to byte vector
    let mut byte_vec: Vec<u8> = Vec::new();
    while !input.is_empty() {
        let cur_byte: String = input.drain(..2).collect();
        byte_vec.push(u8::from_str_radix(&cur_byte, 16).unwrap_or(0));
    }

    // choose algo
    let mut hasher = match algo {
        "sha224"        => HasherEnum::Sha224(Sha224::new()),
        "sha256"        => HasherEnum::Sha256(Sha256::new()),
        "sha384"        => HasherEnum::Sha384(Sha384::new()),
        "sha512"        => HasherEnum::Sha512(Sha512::new()),
        "sha512_224"    => HasherEnum::Sha512_224(Sha512_224::new()),
        "sha512_256"    => HasherEnum::Sha512_256(Sha512_256::new()),
        _               => HasherEnum::Sha512(Sha512::new())
    };

    // call Sha2 lib
    hasher.update(&byte_vec);
    let result = hasher.finish();

    // return result as String
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let input: &str;
    let mut algo = "";

    if args.len() == 2 {
        input = &args[1];
    } else if args.len() == 3 {
        algo = &args[1];
        input = &args[2];
    } else {
        eprintln!("Usage: cargo run [algorithm] <input>");
        eprintln!("Algorithms available: sha224, sha256, sha384, sha512, sha512_224, sha512_256. Default is sha512");
        std::process::exit(1);
    }

    println!("{algo}");
    let hash = calculate_hash(String::from(input),algo);
    println!("{hash}");
}
