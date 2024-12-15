pub mod sha256_definition;
pub mod sha512_definition;
mod sha256_core;
mod sha512_core;

pub trait Hasher {

    fn new() -> Self;

    fn update(&mut self, input: &Vec<u8>);

    fn finish(self) -> Vec<u8>;
}