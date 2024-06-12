use std::env;
use std::fs;

fn main(){
    let args: Vec<String> = env::args().collect();

    println!("Args is {:#?}", &args);

    if args.len() == 1 {
        println!("No arguments supplied");
    } else if args.len() == 2 {
        println!("Error Format");
    }else{
        let config: Config = Config::build(&args);
        println!("This is config: {:#?}", config);
        display(config);
    }
}

#[derive(Debug)]
struct Config<'a>{
    filename: &'a String,
    flags: Vec<&'a String>,
}

impl<'a> Config<'a>{
    fn build(&mut self,args: &'a [String]) -> Self{
        // let mut flags: Vec<String> = Vec::new();
        // let mut filename = String::new();
        for arg in args{
            if arg.starts_with("-"){
                self.flags.push(&arg);
            }else{
                self.filename = &arg;
            }
        }

        self
    }
}

fn display(config: Config){
    for flag in config.flags{
        if flag == "-c"{
            let size = fs::metadata(config.filename.clone()).unwrap().len();
            println!("Size is {:?}", size);
        }
    }
}