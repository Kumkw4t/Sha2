use crate::sha2_lib::sha256_core::Sha256Core;
use crate::sha2_lib::Hasher;

pub struct Sha256 {
    core: Sha256Core,
}

impl Hasher for Sha256 {

    fn new() -> Self {
        Self {
            core: Sha256Core::new([
                0x6a09e667,
                0xbb67ae85,
                0x3c6ef372,
                0xa54ff53a,
                0x510e527f,
                0x9b05688c,
                0x1f83d9ab,
                0x5be0cd19,
            ]),
        }
    }

    fn update(&mut self, input: &Vec<u8>) {
        self.core.update(input);    
    }

    fn finish(self) -> Vec<u8> {
        self.core.finish()
            .iter()
            .flat_map(|x| x.to_be_bytes())
            .collect()
    }
}

pub struct Sha224 {
    core: Sha256Core,
}

impl Hasher for Sha224 {

    fn new() -> Self {
        Self {
            core: Sha256Core::new([
                0xc1059ed8,
                0x367cd507,
                0x3070dd17,
                0xf70e5939,
                0xffc00b31,
                0x68581511,
                0x64f98fa7,
                0xbefa4fa4,
            ]),
        }
    }

    fn update(&mut self, input: &Vec<u8>) {
        self.core.update(input);    
    }

    fn finish(self) -> Vec<u8> {
        self.core.finish()
            .iter()
            .flat_map(|x| x.to_be_bytes())
            .take(28) // 28*8 = 224
            .collect()
    }
}