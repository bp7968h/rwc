use std::fs;
use std::io::{self, Read, Error};

pub fn read_stdin() -> Result<String, Error> {
    let mut std_content = String::new();
    let bytes_read = io::stdin().read_to_string(&mut std_content)?;
    
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No data received via stdin"));
    }

    Ok(std_content)
}

pub fn read_file(file: &String) -> Result<String, Error>{
    fs::read_to_string(file)
}