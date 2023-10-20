# Password manager

## Project information 

- The main goal of this program is to store and encrypt password securely.
- To do this, I will use the Rust programming language, which is one of the best prog language for secure programming.


## Project structure 

- When I think about a password manager that store and encrypt password securely, I first think about the storage(database?). I wan't my program to store multiple password in one place. I can use [serde](https://docs.rs/serde/latest/serde/) to create a file and store the password in this file.
- Then, the encryption part. I will need to choose of one of the encryption I will use to encrypt password. Maybe create mine (?)?


### Storage

[serde](https://docs.rs/serde/latest/serde/) 

- JSON strorage ? 

### Encryption

- sha


### Decomposition

Storage | Encryption
---|---
Create a file to store the password in|Encrypt password using an algorithm
Store password in a file|Check if the password is secure

### Learning 

Storage | Encryption
---|---
How to create a file|How to apply a sha256sum on a string
How to create a serde file .json|How to decrypt the encrypted password
How to encrypt the file himself|How to encrypt the file himself


### Other stuff

- Use a different algorithm for the password
- Use a different name for the file that will contains password
- Check if the password is not empty

1. The user launch the program
2. User type a password to store
3. Store the input of the user 
4. File creation
5. Password encryption
6. In this new file that has been created, write it with the encrypted password
7. Notify the user when it's done

Ask the user the name of the file then the password
Or
Ask the user the password and the file is already created
