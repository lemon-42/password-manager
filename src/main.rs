// -------------- Crypto ----------------

// -------------- File ----------------
//use std::fmt::write;
use std::fs::File;
use std::io::prelude::*;
use std::io;
// -------------- Magic Crypt ----------------
use magic_crypt::{MagicCryptTrait, new_magic_crypt};

fn main() {
    println!("Hi, please the name of the file");

    let mut file_name = String::new();
    io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read file name");
    
    println!("Please now enter a password in the file");
    let mut user_entry = String::new();
    io::stdin()
        .read_line(&mut user_entry)
        .expect("Failed to read input");

    // delete the new line or escape character at the end of file name
    file_name = file_name.trim().to_string();

    if file_name.is_empty() {
        eprintln!("You must choose a name for your file");
        return;
    }

    match create_file(&file_name) {
        Ok(()) => println!("File {} created nicely", file_name),
        Err(err) => eprintln!("Error : {}", err),
    }
    
    let encrypted_content = encrypt_string(&user_entry);
    match write_to_file(&file_name, &encrypted_content) {
        Ok(()) => println!("You write something in the file !"),
        Err(err) => eprintln!("Error : {}", err),
    }
}

// File::create return a Result type, so we need to handle the error
// Result type is an enum with two variants : Ok and Err
// Ok is returned when the operation is successful
// Err is returned when the operation failed
fn create_file(path: &str) -> io::Result<()> {
    let _file = File::create(path)?;
    Ok(())
}

fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    // we need to open the file in write mode, File::create() let us do that
    // also, note that File::open() is used to open a file in read mode
    let mut file = File::create(path)?;
    // we need to write the content as byte in the file because the write_all method
    // take a slice of byte as argument   
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn encrypt_string(content: &str) -> String {
    let mcrypt = new_magic_crypt!("magickey", 256);
    let encrypted_string = mcrypt.encrypt_bytes_to_base64(content);
    encrypted_string
}