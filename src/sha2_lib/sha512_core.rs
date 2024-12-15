pub struct Sha512Core {
    hash_value: [u64;8],
    buffer: Vec<u8>,
}

impl Sha512Core {
    pub const K: [u64;80] = [
        0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
        0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
        0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
        0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
        0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
        0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
        0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
        0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
        0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
        0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
        0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
        0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
        0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
        0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
        0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
        0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
        0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
        0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
        0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
        0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817
    ];

    pub fn new(initial_state: [u64;8]) -> Self {
        Self {
            hash_value: initial_state,
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, input: &Vec<u8>) {

        // input length must be less than 2**64 for sha256        
        assert!((input.len() as u128 *8) < u128::MAX);

        // padding input
        let size: u128 = if input.len() == 1 && input[0] == 0 {0} else {input.len()} as u128;

        if !input.is_empty() && !(input.len() == 1 && input[0] == 0) {
            self.buffer = input.clone();
        }

        self.buffer.push(0b10000000);
        while self.buffer.len() % 128 != 112 {
            self.buffer.push(0);
        }
        self.buffer.extend((size*8).to_be_bytes());

        // verify length and process each 1024 bits of input buffer
        assert!(self.buffer.len() % 128 == 0);
        while !self.buffer.is_empty() {
            let chunk: Vec<u8> = self.buffer.drain(..128).collect();
            self.process(&chunk);
        }
        
    }

    pub fn finish(self) -> [u64;8] {
        // consumes hasher
        self.hash_value
    }

    fn process (&mut self, chunk: &Vec<u8>) {
        
        // compute message schedule
        let mut w = [0u64;80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes([chunk[i*8],chunk[i*8+1],chunk[i*8+2],chunk[i*8+3],chunk[i*8+4],chunk[i*8+5],chunk[i*8+6],chunk[i*8+7]]);
        }

        for i in 16..80 {

            let sigma0: u64 = Self::rotate_right(w[i-15],1) ^ Self::rotate_right(w[i-15],8) ^ (w[i-15]>>7);
            let sigma1: u64 = Self::rotate_right(w[i-2],19) ^ Self::rotate_right(w[i-2],61) ^ (w[i-2]>>6);

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
        for i in 0..80 {
            let sum0: u64 = Self::rotate_right(a, 28) ^ Self::rotate_right(a, 34) ^ Self::rotate_right(a, 39);
            let sum1: u64 = Self::rotate_right(e, 14) ^ Self::rotate_right(e, 18) ^ Self::rotate_right(e, 41);
            let ch:   u64 = (e & f) ^ ((!e) & g);
            let maj:  u64 = (a & b) ^ (a & c) ^ (b & c);

            let t1: u64 = h.wrapping_add(sum1).wrapping_add(ch).wrapping_add(Self::K[i]).wrapping_add(w[i]);
            let t2: u64 = sum0.wrapping_add(maj);

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

    fn rotate_right(var: u64, shift: u64) -> u64 {
        (var >> shift) | (var << (64 - shift))
    }

}