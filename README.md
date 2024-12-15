# README
This is a fully rust implementation of sha2 library. Implements sha256, sha224, sha384, sha512, sha512_256, sha512_224.


## Files

Sha2 is organized as follows:

**Sha2**
|— main.rs
|— sha2_lib
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|— mod.rs
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|— sha256_core.rs
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|— sha512_core.rs
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|— sha256_definition.rs
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;|— sha512_definition.rs

## Use algorithm

Run ```cargo run [algorithm] <hexstring>```

If no algorithm is given, default is sha512.
You can feed the program test vectors from NIST and check the result using stdout. Right now, the program only takes hexstring input. I should add an option in command line to choose if string literal or hex string.

## Implementation details
Each Sha2 algorithm is represented by a **struct** that has one field: either **Sha256Core** or **Sha512Core** depending on inner algorithm used. Sha256Core and Sha512Core have two parameters: _hash_value_, an array of u32 (resp. u64) with the current state of the hash during the computation, and _buffer_ reprensenting the message to hash as a Vec\<u8\>.

Each Sha256Core/Sha512Core struct implements 3 functions : new(), update(input) and finish().

- **New** initialize hash_value with the relevant H0 from NIST documentation
- **Update** computes the padding for the message and updates the buffer accordingly. And then computes the hash value with the relevant algorithm
- **Finish** returns hash_value

Each Sha2 struct then implements the **Hasher** trait giving access to these functions. They simply call them, expect for_finish_ that truncates the results depending on the sha2 variant.

