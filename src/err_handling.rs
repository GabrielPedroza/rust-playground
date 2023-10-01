#![allow(dead_code)]
use std::fs::File;
use std::io::{Write, BufReader, BufRead, ErrorKind};

pub fn err_handling() {

    let path = "hello.txt";

    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }

    };

    write!(output, "Hello, world!\nHow is your day today?").expect("Failed to write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }


}