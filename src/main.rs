use std::process;
use rwc::config::Options;
use rwc::io_utils::{read_file, read_stdin};
use rwc::{get_results, output};


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