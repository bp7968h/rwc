use std::env;
use std::fs;
use std::process;
use std::io;
use std::io::{Error, Read};

fn main(){

    let config = match Options::build() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        }
    };

    let content = if !config.file_name.is_empty() {
        read_file(&config.file_name).unwrap_or_else(|err| {
            eprintln!("File Operation Error: {err}");
            process::exit(1);
        })
    } else {
        read_stdin().unwrap_or_else(|err| {
            eprintln!("Std Io Error: {err}");
            process::exit(1);
        })
    };

    let result: String = get_results(content, &config.flags);

    match output(&result, &config.file_name){
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            process::exit(1);
        }
    }
}

fn read_stdin() -> Result<String, Error> {
    let mut std_content = String::new();
    let bytes_read = io::stdin().read_to_string(&mut std_content)?;
    
    if bytes_read == 0 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No data received via stdin"));
    }

    Ok(std_content)
}

fn read_file(file: &String) -> Result<String, Error>{
    fs::read_to_string(file)
}

fn output(result: &String, file_name: &String) -> Result<(), &'static str> {
    if result.is_empty(){
        return Err("Wrong flags!!");
    }
    println!("{} {}", result, file_name);
    Ok(())
}

#[derive(Debug)]
struct Options {
    file_name: String,
    flags: Vec<String>,
}

impl Options {
    fn build() -> Result<Options, &'static str> {
        let args: Vec<String> = env::args().collect();

        let mut file_name: String = String::new();
        let mut flags: Vec<String> = Vec::new();

        for arg in args[1..].iter(){
            if arg.starts_with("-") {
                flags.push(arg.clone());
            }else {
                file_name = arg.clone();
            }
        }

        Ok(Options {file_name, flags})
    }
}


fn get_results(content: String, flags: &[String]) -> String {
    let mut result: String = String::new();
    if flags.is_empty(){
        result.push_str(&format!("{} {} {}", content.lines().count(), content.split_whitespace().count(), content.len()));
        return result
    }
    for flag in flags {
        match flag.as_str() {
            "-c" => result.push_str(&format!(" {:?}", content.len())),
            "-l" => result.push_str(&format!(" {:?}", content.lines().count())),
            "-w" => result.push_str(&format!(" {:?}", content.split_whitespace().count())),
            "-m" => result.push_str(&format!(" {:?}", content.chars().count())),
            _ => ()
        }
    }
    result
}

