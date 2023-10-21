use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use orion::hazardous::stream::chacha20;
use argon2::{self, Argon2, password_hash::{SaltString, PasswordHasher}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("[-] Usage : ./pmon <file> ");
        return Err("Invalid number of arguments".into())
    }

    let file_path = args.get(1).unwrap();
    println!("[+] Using file : {}", file_path);

    if !file_path.is_empty() {
        let user_password = prompt_for_password()?;
        if user_password.trim().is_empty() {
            return Err("You must specify a password.".into())
        }

        let key = derive_key_from_password(&user_password);
        let encrypt_password = encrypt_password(&user_password, &key);

        let mut file = File::create(file_path)?;

        file.write_all(&key)?;
        file.write_all(&encrypt_password)?;
        println!("[+] Password save to {}", file_path);
    } else {
        println!("[+] File path not specified. Creating one of your own : ");
        let mut file_name = String::new();
        io::stdin().read_line(&mut file_name)?;
        create_file(&file_name.trim())?;
        println!("File created nicely.")
    }
    Ok(())
}

fn prompt_for_password() -> Result<String, io::Error> {
    println!("[+] Enter a password for your file : ");
    let mut user_password = String::new();
    io::stdin().read_line(&mut user_password)?;
    Ok(user_password)
}

fn create_file(path: &str) -> io::Result<()> {
    let _file = File::create(path)?;
    Ok(())
}

#[warn(dead_code)]
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

fn derive_key_from_password(password: &str) -> [u8; 32] {
    let salt= SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();

    let hash = password_hash.hash.unwrap();
    let mut key = [0u8; 32];
    key.copy_from_slice(hash.as_bytes());
    key
}

fn save_encrypted_password(file_path: &str, salt: &[u8], encrypted_password: &[u8]) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(salt)?;
    file.write_all(encrypted_password)?;
    Ok(())
}

fn encrypt_password(password: &str, key: &[u8]) -> Vec<u8> {
    let password_byte = password.as_bytes();
    let nonce = chacha20::Nonce::from_slice(&[0u8; 12]).unwrap(); // use a unique nonce for each cypher

    let mut ciphertext = vec![0u8; password_byte.len()];

    chacha20::encrypt(&chacha20::SecretKey::from_slice(key).unwrap(),
        &nonce, 
    0, 
    password_byte, 
    &mut ciphertext
    ).expect("Encryption failed.");

    ciphertext
}