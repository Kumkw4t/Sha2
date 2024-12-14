pub struct Sha256Core {
    state: [u32;8],
    buffer: Vec<u8>,
}

impl Sha256Core {
    const K: [u32;64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xb5c0fbcf, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
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
            state: initial_state,
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, input: Vec<u8>) {

        // input length must be less than 2**64 for sha256        
        assert!(input.len() < 0xffffffff);

        // padding input
        let size: usize = input.len();
        self.buffer = input;

        self.buffer.push(0b10000000);
        while self.buffer.len() % 64 != 56 {
            self.buffer.push(0);
        }
        self.buffer.extend((size*8).to_be_bytes());

        // verify length and process each 512 bits of input buffer
        assert!(self.buffer.len() % 64 == 0);
        let cur_length = self.buffer.len();
        println!("buffer lenght before / after : {size}/{cur_length}");
        let mut i = 0;
        while !self.buffer.is_empty() {
            let chunk: Vec<u8> = self.buffer.drain(..64).collect();
            println!("{i}eme: {chunk:?}");
            self.process(&chunk);
            i+=1;
        }
        
    }

    fn process (&mut self, chunk: &Vec<u8>) {
        // TODO
    }

    pub fn finish(self) -> [u32;8] {
        // consumes hasher
        [0,0,0,0,0,0,0,0]
    }
}