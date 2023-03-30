use std::io::{self, Write};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    for arg in args.iter() {
        print!("{}", arg);
    }
    getinput();
}

fn getinput() {
    loop {
        // print a pseudo prompt
        print!("> ");
        let _ = io::stdout().flush();
        // create a buffer to hold the input
        let mut input = String::new();
        // read from stdin
        let _ = io::stdin().read_line(&mut input);
        // print the input
        println!("You said: {} ", input);

        // break loop if user enters "exit"
        if input.trim_end().eq_ignore_ascii_case("exit") {
            break;
        }
    }

}
