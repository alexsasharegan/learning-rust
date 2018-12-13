use std::env;
use std::process;

struct ProgramOptions {
    help: bool,
    overwrite: bool,
    verbose: bool,
}

impl ProgramOptions {
    fn new() -> ProgramOptions {
        ProgramOptions {
            help: false,
            overwrite: true,
            verbose: false,
        }
    }
}

fn main() {
    let mut argv = env::args().collect::<Vec<String>>();
    let cmd = argv.remove(0);

    let mut options = ProgramOptions::new();
    let mut positional = Vec::new();

    for s in argv {
        if s.starts_with("-") {
            for c in s.chars().skip(1) {
                match c {
                    'h' => {
                        options.help = true;
                    }
                    'v' => {
                        options.verbose = true;
                    }
                    'n' => {
                        options.overwrite = false;
                    }
                    _ => {
                        eprintln!("Found unsupported flag: '{}'", c);
                        process::exit(1)
                    }
                }
            }
        } else {
            positional.push(s);
        }
    }

    println!("Command: {:?}", cmd);
    println!("Args: {:?}", positional);
    println!("Verbose: {}", options.verbose);
    println!("Overwrite: {}", options.overwrite);
    println!("Help: {}", options.help);
}
