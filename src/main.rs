use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

fn main() {
    if let Ok(lines) = read_lines("settings.gradle") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(val) = line {
                println!("{}", val)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_package(val: &str) -> &str {
    if val.contains("instrumentation") {
        let i = val.find(':');
        if i.is_some() {
            let j = i.unwrap();
            //println!("{}", val.to_string().substring(j + 1, val.len() - 1));
            let start = j + 1;
            let end = val.len() - 1;
            let pkg = val.substring(start, end);
            return pkg
        }
    }
    ""
}

#[cfg(test)]
mod tests {
    use crate::extract_package;

    #[test]
    fn blank_is_blank() {
        let actual = extract_package("");
        assert_eq!(actual, "");
    }

    #[test]
    fn it_works() {
        //let actual = extract_package(&"include 'instrumentation:akka-2.2'");
        //assert_eq!(actual, Some("akka-2.2"));
    }
}