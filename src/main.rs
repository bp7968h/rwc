use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();

    println!("Args is {:#?}", args);

    if args.len() == 1 {
        println!("No arguments supplied");
    } else if args.len() == 2 {
        println!("Error Format");
    }else{
        let config: Config = Config::build(&args);
        println!("This is config: {:#?}", config)
    }
}

#[derive(Debug)]
struct Config{
    filename: String,
    flags: Vec<String>,
}

impl Config{
    fn build(args: &[String]) -> Config{
        let mut flags: Vec<String> = Vec::new();
        let mut filename = String::new();
        for arg in args{
            if arg.starts_with("-"){
                flags.push(arg.clone());
            }else{
                filename = arg.clone();
            }
        }

        Config {filename, flags}
    }
}