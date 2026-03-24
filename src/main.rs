#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");

    // Rust doesn't flush afrer a print! only after println! so the print above won't be flushed
    // unless we force it
    io::stdout().flush().unwrap();

    let mut raw_cmd = String::new();

    io::stdin()
        .read_line(&mut raw_cmd)
        .expect("Failed to read user input");

    // let cmd = raw_cmd.trim();

    println!("{}: command not found", raw_cmd.trim());
}
