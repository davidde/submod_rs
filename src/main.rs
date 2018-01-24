use std::env;

fn help() {
    println!("
Submod

Usage: submod inputfile seconds
  inputfile: .srt or .vtt subtitle file
  seconds: seconds to add or subtract from time encoding
");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        help();
        return;
    }
    
    let filename = &args[1];
    let seconds = &args[2];
    let seconds: f64 = match seconds.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("Error: second argument not a number");
            help();
            return;
        },
    };
    
    println!("Change file {} with {} seconds", filename, seconds);

}
