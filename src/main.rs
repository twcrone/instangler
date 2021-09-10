use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;
use std::cmp::Ordering;

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
        pkgs.sort_by(|a, b| cmp_as_num_if_possible(a, b));
        println!("{}", "// Loaded instrumentation modules");
        for a in &pkgs {
            println!("Supportability/WeaveInstrumentation/Loaded/com.newrelic.instrumentation.{}/1", a);
        }
        println!();
        println!("{}", "// Skipped instrumentation modules");
        for b in &pkgs {
            println!("Supportability/WeaveInstrumentation/Skipped/com.newrelic.instrumentation.{}/1", b);
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

fn cmp_as_num_if_possible(a: &str, b: &str) -> Ordering {
    let i = a.parse::<u8>();
    let j = b.parse::<u8>();
    if i.is_ok() && j.is_ok() {
        let ai = i.unwrap();
        let bi = j.unwrap();
        ai.cmp(&bi)
    } else {
        a.cmp(b)
    }
}

#[cfg(test)]
mod tests {
    use crate::{extract_package, cmp_as_num_if_possible};
    use std::cmp::Ordering::{Less, Greater, Equal};

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

    #[test]
    fn compare_as_strings() {
        assert_eq!(cmp_as_num_if_possible("java", "javax"), Less);
        assert_eq!(cmp_as_num_if_possible("javax", "java"), Greater);
        assert_eq!(cmp_as_num_if_possible("java", "java"), Equal);
    }

    #[test]
    fn compare_as_ints() {
        assert_eq!(cmp_as_num_if_possible("1", "2"), Less);
        assert_eq!(cmp_as_num_if_possible("2", "1"), Greater);
        assert_eq!(cmp_as_num_if_possible("1", "1"), Equal);
        assert_eq!(cmp_as_num_if_possible("30", "4"), Greater);
        //assert_eq!(cmp_as_num_if_possible("1", "2"), Greater);
        //assert_eq!(cmp_as_num_if_possible("java", "java"), Equal);
    }
}