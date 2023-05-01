use libaes::Cipher;
use std::env;
use std::io::prelude::*;
use std::fs;
use std::net::TcpStream;



fn main() -> std::io::Result<()> {

    let my_key = b"ThisIs16BytesKey";

    let iv = b"ThisIs16BytesIV_";
    
    let mut stream = TcpStream::connect("127.0.0.1:1337").unwrap();

    let cipher = Cipher::new_128(my_key);
    let data = fs::read("/bin/id").unwrap();
    // Encryption
    let encrypted = cipher.cbc_encrypt(iv, &data);    
    stream.write(&encrypted);

    Ok(())

}

