use crate::sha256_core::Sha256Core;

pub struct Sha256 {
    core: Sha256Core,
}

impl Sha256 {

    pub fn new() -> Self {
        Self {
            core: Sha256Core::new([
                0x6a09e667,
                0xbb67ae85,
                0x3c6ef372,
                0xa54ff53a,
                0x510e527f,
                0x9b05688c,
                0x1f83d9ab,
                0x5be0cd19
            ]),
        }
    }

    pub fn update(&mut self, input: Vec<u8>) {
        self.core.update(input);    
    }

    pub fn finish(self) -> [u32;8] {
        self.core.finish()
    }
}