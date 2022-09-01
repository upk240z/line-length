use std::fs::File;
use std::io::{BufReader, Read};
use std::{env, process};

fn show_line_length(path: &String) {
    match File::open(path) {
        Ok(file) => {
            let mut no = 0;
            let mut counter = 0;
            for result in BufReader::new(file).bytes() {
                match result {
                    Ok(byte) => {
                        if byte == 0x0a || byte == 0x0d {
                            if counter == 0 {
                                continue;
                            }
                            no += 1;
                            println!("{}:{}:{}", path, no, counter);
                            counter = 0;
                        } else {
                            counter += 1;
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        break;
                    }
                }
            }

            if counter > 0 {
                println!("{}:{}", no + 1, counter);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("usage: {} <file>", args.get(0).unwrap());
        process::exit(1);
    }

    for (index, path) in args.iter().enumerate() {
        if index == 0 {
            continue;
        }
        show_line_length(path);
    }
}
