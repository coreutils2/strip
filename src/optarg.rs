extern crate getopts;

use self::getopts::Options;
use std::env;

pub struct OptArg {
    opts: getopts::Options,
}

impl OptArg {
    pub fn print_usage(&self) {
        let program = env::args().take(1).next().unwrap();
        let brief = format!("Usage: {} [options]", &program);
        print!("{}", self.opts.usage(&brief));
    }

    pub fn parse(&self) -> getopts::Result {
        let args: Vec<String> = env::args().skip(1).collect();
        return self.opts.parse(&args);
    }

    pub fn new() -> OptArg {
        let mut opts = Options::new();
        opts.optflag("h", "help", "print this help menu");
        opts.optopt("l", "leading", "remove leading characters", "STRING");
        opts.optopt("t", "trailing", "remove trailing charactes", "STRING");
        return OptArg { opts: opts };
    }
}
