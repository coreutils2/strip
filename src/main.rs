#[macro_use]
extern crate clap;
use clap::{App, Arg};
use std::io;
use std::io::BufRead;



fn strip(leading: &str, trailing: &str) {
    let stdin = io::stdin();
    // Each line is doesn't have a newline byte (the 0xA byte)
    // or CRLF (0xD, 0xA bytes) at the end.
    for line in stdin.lock().lines() {
        println!(
            "{}",
            line.unwrap()
                .trim() // leading and trailing whitespaces are always remove
                .trim_left_matches(leading)
                .trim_right_matches(trailing)
        )
    }
}


fn main() {
    let app = App::new(format!(
        "{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_AUTHORS")
    )).version(crate_version!());

    let options = app.arg(
        Arg::with_name("leading")
            .short("l")
            .long("leading")
            .value_name("TEXT")
            .help("remove leading characters")
            .takes_value(true)
            .required(false),
    ).arg(
            Arg::with_name("trailing")
                .short("t")
                .long("trailing")
                .value_name("TEXT")
                .help("remove trailing characters")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let leading = options.value_of("leading").unwrap_or("");
    let trailing = options.value_of("trailing").unwrap_or("");
    strip(&leading, &trailing);
}
