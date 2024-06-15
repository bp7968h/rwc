pub mod config;
pub mod io_utils;

pub fn get_results(content: String, flags: &[String]) -> String {
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

pub fn output(result: &String, file_name: &String) -> Result<(), &'static str> {
    if result.is_empty(){
        return Err("Wrong flags!!");
    }
    println!("{} {}", result, file_name);
    Ok(())
}