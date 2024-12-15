use crate::sha512_core::Sha512Core;

pub struct Sha512 {
    core: Sha512Core,
}

impl Sha512 {

    pub fn new() -> Self {
        Self {
            core: Sha512Core::new([
                0x6a09e667f3bcc908,
                0xbb67ae8584caa73b,
                0x3c6ef372fe94f82b,
                0xa54ff53a5f1d36f1,
                0x510e527fade682d1,
                0x9b05688c2b3e6c1f,
                0x1f83d9abfb41bd6b,
                0x5be0cd19137e2179
            ]),
        }
    }

    pub fn update(&mut self, input: &Vec<u8>) {
        self.core.update(input);    
    }

    pub fn finish(self) -> [u64;8] {
        self.core.finish()
    }
}