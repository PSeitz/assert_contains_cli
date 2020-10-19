use std::env;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: String = env::args().skip(1).collect::<Vec<String>>().join(" ");
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    if buffer.contains(&args) {
        eprintln!("error: {:?} found", args);
        process::exit(0x0100);
    }
}
