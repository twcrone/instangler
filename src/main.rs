use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

fn main() {
    if let Ok(lines) = read_lines("settings.gradle") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                if val.contains("instrumentation") {
                    let i = val.find(':');
                    if i.is_some() {
                        let j = i.unwrap();
                        println!("{}", val.to_string().substring(j + 1, val.len() - 1));
                    }
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
