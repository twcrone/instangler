use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;

fn main() {
    if let Ok(lines) = read_lines("settings.gradle") {
        let mut pkgs = Vec::new();
        for line in lines {
            if let Ok(val) = line {
                let pkg = extract_package(&val);
                if pkg.is_some() {
                    pkgs.push(pkg.unwrap().to_string());
                }
            }
        }
        for x in pkgs {
            println!("{}", x);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_package(val: &str) -> Option<&str> {
    if val.contains("instrumentation") {
        let i = val.find(':');
        if i.is_some() {
            let j = i.unwrap();
            let start = j + 1;
            let end = val.len() - 1;
            let pkg = val.substring(start, end);
            return Some(pkg)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::extract_package;

    #[test]
    fn blank_is_blank() {
        let actual = extract_package("");
        assert_eq!(actual, None);
    }

    #[test]
    fn it_works() {
        let actual = extract_package("include 'instrumentation:akka-2.2'");
        assert_eq!(actual, Some("akka-2.2"));
    }
}