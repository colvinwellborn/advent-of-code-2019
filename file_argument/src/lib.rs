use std::env;

pub fn only_file_arg(name: &str) -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("USAGE: {} <file>", name);
        std::process::exit(1);
    }
    args[1].to_string()
}
