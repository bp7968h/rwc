use std::env;

#[derive(Debug)]
pub struct Options {
    pub file_name: String,
    pub flags: Vec<String>,
}

impl Options {
    pub fn build() -> Result<Options, &'static str> {
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