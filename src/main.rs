use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("[-] Usage : ./pmon <file> ")
    }

    let file_path = &args[1];
    println!("[+] Using file : {}", file_path);

    let mut user_password = String::new();
    if !file_path.is_empty() {
        println!("[+] Enter a password in your file : ");
        io::stdin()
            .read_line(&mut user_password)
            .expect("Failed to read input");

        if user_password.trim().is_empty() {
            eprintln!("[-] You must specify a password.");
            return;
        }

        let result = write_to_file(file_path, &user_password);
        match result {
            Ok(()) => println!("[+] File created nicely"),
            Err(err) => eprintln!("[-] Error when creating the file : {}", err),
        } 

    } else {
        println!("[+] File path not specified. Creating one of your own : ");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read input");

        match create_file(&file_name) {
            Ok(()) => println!("We're in the match section !"),
            Err(err) => eprintln!("Bug in the match : {}", err),
        }
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
    // open the file in read and write acces, and open the file if it exist
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)?;

    // read the content of the file that we want to open
    // existing_content will now contains the content of the file before writing in it
    let mut existing_content = String::new();
    file.read_to_string(&mut existing_content)?;

    // push at the end of the file
    existing_content.push_str(content);

    // redirect the position for reading at the start of the file
    file.seek(io::SeekFrom::Start(0))?;
    // we need to write the content as byte in the file because the write_all method
    // take a slice of byte as argument
    file.write_all(existing_content.as_bytes())?;
    Ok(())
}

// fn encrypt_string(content: &str) -> String {
//     let mcrypt = new_magic_crypt!("magickey", 256);
//     let encrypted_string = mcrypt.encrypt_bytes_to_base64(content);
//     encrypted_string
// }
