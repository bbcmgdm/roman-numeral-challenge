#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::{env, process};

lazy_static! {
    static ref VALUE_MAP: HashMap<&'static str, u64> = {
        let mut m = HashMap::new();
        m.insert("CM", 900);
        m.insert("M", 1000);
        m.insert("CD", 400);
        m.insert("D", 500);
        m.insert("XC", 90);
        m.insert("C", 100);
        m.insert("XL", 40);
        m.insert("L", 50);
        m.insert("IX", 9);
        m.insert("X", 10);
        m.insert("IV", 4);
        m.insert("V", 5);
        m.insert("I", 1);
        m
    };
    static ref KEY_ORDER: Vec<&'static str> =
        vec!["CM", "M", "CD", "D", "XC", "C", "XL", "L", "IX", "X", "IV", "V", "I"];
}

fn is_valid(input: &str) -> bool {
    lazy_static! {
        static ref NUMERALS_RE: Regex =
            Regex::new("^M{0,4}(CM|CD|D?C{0,3})(XC|XL|L?X{0,3})(IX|IV|V?I{0,3})$").unwrap();
    }

    NUMERALS_RE.is_match(input)
}

fn convert(input: &str) -> Result<u64, ()> {
    let mut result = 0;
    let mut intermediate = String::from(input);

    for key in KEY_ORDER.iter() {
        let value = VALUE_MAP.get(key).unwrap();

        while let Some(pos) = intermediate.find(key) {
            result += value;
            intermediate.replace_range(pos..(pos + key.len()), "");
        }
    }

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        eprintln!("Usage: {} VALUE", args[0]);
        process::exit(1);
    }

    if !is_valid(&args[1]) {
        eprintln!("Invalid Roman numeral {}", args[1]);
        process::exit(2);
    }

    if let Ok(result) = convert(&args[1]) {
        println!("{:?}", result);
    } else {
        eprintln!("Failed to convert Roman numeral {}", args[1]);
        process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_validate() {
        assert_eq!(is_valid("I"), true);
        assert_eq!(is_valid("II"), true);
        assert_eq!(is_valid("III"), true);
        assert_eq!(is_valid("V"), true);
        assert_eq!(is_valid("VI"), true);
        assert_eq!(is_valid("VII"), true);
        assert_eq!(is_valid("VIII"), true);
        assert_eq!(is_valid("X"), true);
        assert_eq!(is_valid("XX"), true);
        assert_eq!(is_valid("XXX"), true);
        assert_eq!(is_valid("C"), true);
        assert_eq!(is_valid("CC"), true);
        assert_eq!(is_valid("CCC"), true);
        assert_eq!(is_valid("M"), true);
        assert_eq!(is_valid("MM"), true);
        assert_eq!(is_valid("MMM"), true);
        assert_eq!(is_valid("IV"), true);
        assert_eq!(is_valid("IX"), true);
        assert_eq!(is_valid("XI"), true);
        assert_eq!(is_valid("L"), true);
        assert_eq!(is_valid("XL"), true);
        assert_eq!(is_valid("D"), true);
        assert_eq!(is_valid("CD"), true);
        assert_eq!(is_valid("CM"), true);
        assert_eq!(is_valid("MCMXCIX"), true);

        assert_eq!(is_valid("IIII"), false);
        assert_eq!(is_valid("XXXX"), false);
        assert_eq!(is_valid("VV"), false);
        assert_eq!(is_valid("LL"), false);
        assert_eq!(is_valid("DD"), false);
        assert_eq!(is_valid("IIX"), false);
        assert_eq!(is_valid("ABCDE"), false);
        assert_eq!(is_valid("ICXXXXIIVV"), false);
    }

    #[test]
    fn test_convert() {
        assert_eq!(convert("I"), Ok(1));
        assert_eq!(convert("II"), Ok(2));
        assert_eq!(convert("III"), Ok(3));
        assert_eq!(convert("V"), Ok(5));
        assert_eq!(convert("VI"), Ok(6));
        assert_eq!(convert("VII"), Ok(7));
        assert_eq!(convert("VIII"), Ok(8));
        assert_eq!(convert("X"), Ok(10));
        assert_eq!(convert("XX"), Ok(20));
        assert_eq!(convert("XXX"), Ok(30));
        assert_eq!(convert("C"), Ok(100));
        assert_eq!(convert("CC"), Ok(200));
        assert_eq!(convert("CCC"), Ok(300));
        assert_eq!(convert("M"), Ok(1000));
        assert_eq!(convert("MM"), Ok(2000));
        assert_eq!(convert("MMM"), Ok(3000));
        assert_eq!(convert("IV"), Ok(4));
        assert_eq!(convert("IX"), Ok(9));
        assert_eq!(convert("XI"), Ok(11));
        assert_eq!(convert("L"), Ok(50));
        assert_eq!(convert("XL"), Ok(40));
        assert_eq!(convert("D"), Ok(500));
        assert_eq!(convert("CD"), Ok(400));
        assert_eq!(convert("CM"), Ok(900));
        assert_eq!(convert("MCMXCIX"), Ok(1999));
    }
}
