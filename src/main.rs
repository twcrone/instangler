use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use substring::Substring;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal};

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
        pkgs.sort_by(|a, b| cmp_pkgs(a, b));
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
        //println!("{} {}", ai, bi);
        ai.cmp(&bi)
    } else {
        a.cmp(b)
    }
}

fn package_wo_version(s: &str) -> &str {
    let rfind = s.rfind("-");
    if rfind.is_some() {
        s.substring(0, rfind.unwrap())
    }
    else {
        s
    }
}

fn cmp_pkgs(a: &str, b: &str) -> Ordering {
    if a == b {
        return Equal
    }
    else {
        let asub = package_wo_version(a);
        let bsub = package_wo_version(b);
        //println!("*{} {}", asub, bsub);
        let first_order = asub.cmp(bsub);
        if first_order != Equal {
            first_order
        }
        else {
            let ari = a.rfind("-");
            if ari.is_none() {
                return cmp_as_num_if_possible(a, b);
            }
            let start = ari.unwrap();
            let asub = a.substring(start + 1, a.len());
            let bsub = b.substring(start + 1, b.len());
            // println!("{} {}", asub, bsub);
            let mut a_split = asub.split(".");
            let mut b_split = bsub.split(".");
            let mut ai = a_split.nth(0);
            let mut bi = b_split.nth(0);
            let mut order: Ordering;
            loop {
                // println!("In loop i= {}", i);
                if ai.is_none() || bi.is_none() {
                    return Equal
                }
                // println!("Not Equal");
                order = cmp_as_num_if_possible(ai.unwrap(), bi.unwrap());
                if !order.is_eq() {
                    return order
                }
                ai = a_split.nth(0);
                bi = b_split.nth(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{extract_package, cmp_as_num_if_possible, cmp_pkgs};
    use std::cmp::Ordering::{Less, Greater, Equal};

    #[test]
    fn base_package_compare() {
        let first = "com.newrelic.instrumentation.scala-2.9.3";
        let last = "com.newrelic.instrumentation.zio";
        assert_eq!(cmp_pkgs(first, last), Less);
    }

    #[test]
    fn edge_package_compare() {
        let first = "com.newrelic.instrumentation.play-shaded-async-http-client-1.0.0";
        let last = "com.newrelic.instrumentation.play-ws-2.6.0";
        assert_eq!(cmp_pkgs(first, last), Less);
    }

    // #[test]
    // fn another_edge_package_compare() {
    //     let first = "com.newrelic.instrumentation.jdbc-postgresql-8.0-312.jdbc3";
    //     let last = "com.newrelic.instrumentation.jdbc-postgresql-9.4.1208";
    //     assert_eq!(cmp_pkgs(first, last), Less);
    // }

    #[test]
    fn package_compare() {
        let old = "com.newrelic.instrumentation.scala-2.9.3";
        let new = "com.newrelic.instrumentation.scala-2.13.0";
        assert_eq!(cmp_pkgs(new, new), Equal);
        assert_eq!(cmp_pkgs(old, new), Less);
        assert_eq!(cmp_pkgs(new, old), Greater);
    }

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
    }
}