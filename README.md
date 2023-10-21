## Project information 

- The main goal of this program is to store and encrypt password securely.

## Usage

```bash
./pmon file_to_create_or_use
```

## TODO

- Find a better thing than b64..
- Encrypt the file himself
- Decrypt a line in a file
- Use arguments to select an already created file

### Storage

- [serde](https://docs.rs/serde/latest/serde/) 

- JSON strorage ? 

### Encryption

- [ring](https://docs.rs/ring/latest/ring/)
- [orion](https://github.com/orion-rs/orion)