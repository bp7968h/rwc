use std::env;
use std::fs;
use std::process;
use std::io::{Error};

fn main(){
    let args = get_arguments().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let config : FlagsFile = parse_arguments(&args[1..]);

    if config.file_name.is_empty() {
        eprintln!("No file specified");
        process::exit(1);
    }

    let file_content: String = read_file(&config.file_name).unwrap_or_else(|err| {
        eprintln!("File Operation Error: {err}");
        process::exit(1);
    });

    let result: String = get_results(file_content, &config.flags);

    if result.is_empty(){
        eprintln!("Wrong flags!!");
        process::exit(1);
    }
    println!("{} {}", result, config.file_name);
    process::exit(0);
}

fn read_file(file: &String) -> Result<String, Error>{
    fs::read_to_string(file)
}

fn get_arguments() -> Result<Vec<String>, &'static str>{
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Err("Not enough Parameters")
    }
    Ok(args)
}

#[derive(Debug)]
struct FlagsFile {
    file_name: String,
    flags: Vec<String>,
}

fn parse_arguments(args: &[String]) -> FlagsFile {
    let mut file_name: String = String::new();
    let mut flags: Vec<String> = Vec::new();

    for arg in args{
        if arg.starts_with("-") {
            flags.push(arg.clone());
        }else {
            file_name = arg.clone();
        }
    }

    FlagsFile {file_name, flags}
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

    // println!("{}", result);
    result
}

