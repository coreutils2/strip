mod optarg;
use std::io;
use optarg::OptArg;
use std::io::BufRead;
use std::process;

fn strip(leading: &String, trailing: &String) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!(
            "{}",
            line.unwrap()
                .trim_left_matches(leading)
                .trim_right_matches(trailing)
        )
    }
}

fn main() {
    let optarg = OptArg::new();
    let options = match optarg.parse() {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f);
            process::exit(1);
        }
    };

    if options.opt_present("help") {
        optarg.print_usage();
        process::exit(0);
    }

    let leading: String = match options.opt_str("leading") {
        Some(x) => x,
        None => String::new(),
    };

    let trailing: String = match options.opt_str("trailing") {
        Some(x) => x,
        None => String::new(),
    };

    strip(&leading, &trailing);
}
