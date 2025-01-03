pub struct Sha256Core {
    hash_value: [u32;8],
    buffer: Vec<u8>,
}

impl Sha256Core {
    pub const K: [u32;64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
    ];

    pub fn new(initial_state: [u32;8]) -> Self {
        Self {
            hash_value: initial_state,
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, input: &Vec<u8>) {

        // input length must be less than 2**64 for sha256        
        assert!((input.len() as u64*8) < u64::MAX);

        // padding input
        let size: u64 = if input.len() == 1 && input[0] == 0 {0} else {input.len()} as u64;
        if !input.is_empty() && !(input.len() == 1 && input[0] == 0) {
            self.buffer = input.clone();
        }

        self.buffer.push(0b10000000);
        while self.buffer.len() % 64 != 56 {
            self.buffer.push(0);
        }
        self.buffer.extend((size*8).to_be_bytes());

        // verify length and process each 512 bits of input buffer
        assert!(self.buffer.len() % 64 == 0);
        while !self.buffer.is_empty() {
            let chunk: Vec<u8> = self.buffer.drain(..64).collect();
            self.process(&chunk);
        }
        
    }

    pub fn finish(self) -> [u32;8] {
        // consumes hasher
        self.hash_value
    }

    fn process (&mut self, chunk: &Vec<u8>) {
        
        // compute message schedule
        let mut w = [0u32;64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([chunk[i*4], chunk[i*4+1], chunk[i*4+2], chunk[i*4+3]]);
        }

        for i in 16..64 {

            let sigma0: u32 = Self::rotate_right(w[i-15],7) ^ Self::rotate_right(w[i-15],18) ^ (w[i-15]>>3);
            let sigma1: u32 = Self::rotate_right(w[i-2],17) ^ Self::rotate_right(w[i-2],19) ^ (w[i-2]>>10);

            w[i] = sigma1.wrapping_add(w[i-7]).wrapping_add(sigma0).wrapping_add(w[i-16]);
        }

        // initiate working variables with current hash value
        let mut a = self.hash_value[0];
        let mut b = self.hash_value[1];
        let mut c = self.hash_value[2];
        let mut d = self.hash_value[3];
        let mut e = self.hash_value[4];
        let mut f = self.hash_value[5];
        let mut g = self.hash_value[6];
        let mut h = self.hash_value[7];

        // process message schedule
        for i in 0..64 {
            let sum0: u32 = Self::rotate_right(a, 2) ^ Self::rotate_right(a, 13) ^ Self::rotate_right(a, 22);
            let sum1: u32 = Self::rotate_right(e, 6) ^ Self::rotate_right(e, 11) ^ Self::rotate_right(e, 25);
            let ch:   u32 = (e & f) ^ ((!e) & g);
            let maj:  u32 = (a & b) ^ (a & c) ^ (b & c);

            let t1: u32 = h.wrapping_add(sum1).wrapping_add(ch).wrapping_add(Self::K[i]).wrapping_add(w[i]);
            let t2: u32 = sum0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }

        // updating hash value before moving to next chunk
        for i in 0..8 {
            self.hash_value[i] = self.hash_value[i].wrapping_add([a, b, c, d, e, f, g, h][i]);
        }
    }

    fn rotate_right(var: u32, shift: u32) -> u32 {
        (var >> shift) | (var << (32 - shift))
    }

}