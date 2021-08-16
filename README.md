## rust-base64-encoder

A simple, learning purpose, base64 encoder/decoder made with Rust following this [tutorial](https://dev.to/tiemen/implementing-base64-from-scratch-in-rust-kb1)

To use is simple:

1. Clone this repo with: `git clone git@github.com:EclesioMeloJunior/rust-base64-encoder.git`
2. With rust installed run: `cargo build`
3. After that you can run: `echo "to encode string" | cargo run -- encode`
4. To decode you just need: `echo "encoded string" | cargo run -- decode`