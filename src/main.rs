use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

static HELLO_STR: &str = "hello\n";

fn main() {
    let path = Path::new("hello.txt");

    let mut f = match File::create(&path) {
        Err(why)    => panic!("{}", why),
        Ok(f)       => f,
    };

    match f.write_all(HELLO_STR.as_bytes()) {
        Err(why)    => panic!("{}", why),
        Ok(_)       => print!(""),
    };

    f = match File::open(&path) {
        Err(why)    => panic!("{}", why),
        Ok(f)       => f,
    };

    let mut read_str = String::new();

    match f.read_to_string(&mut read_str) {
        Err(why)    => panic!("{}", why),
        Ok(_)       => print!("{}", read_str),
    };
}

