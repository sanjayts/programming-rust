use regex::Regex;
use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter, Write};
use std::hash::{Hash, Hasher};
use std::ptr::hash;
use std::rc::Rc;
use std::str::FromStr;

#[test]
fn test_string_collect() {
    let s = "Man hat tan";
    let city: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    assert_eq!(city, "Manhattan");
}

#[test]
fn test_write() {
    let mut s = String::new();

    write!(s, "Hey, my name is {}", "Rand").unwrap();
    assert_eq!(s, "Hey, my name is Rand");

    let mut start = "Man ".to_string();
    let end = " Middle".to_string();
    start = start + "In The" + &end;
    assert_eq!(start, "Man In The Middle");
}

#[test]
fn test_drain() {
    let mut name = "Callindor".to_string();
    name.drain(..4);
    assert_eq!(name, "indor");
}

#[test]
fn test_replace() {
    let s = "Dungeons and Dragons".to_string();
    let s = s.replace('D', "G");
    assert_eq!(s, "Gungeons and Gragons");

    let s = s.replace(|c| c == 'D', "G");
    assert_eq!(s, "Gungeons and Gragons");
}

#[test]
fn test_split() {
    let line = "name,age,address,salary,";

    let parts = line.split(',').collect::<Vec<_>>();
    assert_eq!(parts, vec!["name", "age", "address", "salary", ""]);

    let parts = line.split_terminator(',').collect::<Vec<_>>();
    assert_eq!(parts, vec!["name", "age", "address", "salary"]);
}

#[test]
fn test_trim() {
    let line = "xasydsdf1234";
    let trimmed = line.trim_matches(char::is_alphabetic);
    assert_eq!(trimmed, "1234");
}

#[test]
fn test_from_string() {
    assert_eq!(usize::from_str("123"), Ok(123 as usize));
    assert_eq!(f64::from_str("123"), Ok(123_f64));
    assert_eq!(
        f64::from_str("123_a").unwrap_err().to_string(),
        "invalid float literal"
    );
}

#[derive(Default)]
struct Person {
    name: String,
    age: usize,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person<name={}, age={}>", self.name, self.age)
    }
}

#[test]
fn test_display() {
    let p = Person {
        name: "san".to_string(),
        age: 21,
    };

    assert_eq!(format!("{}", p), "Person<name=san, age=21>");
    assert_eq!(p.to_string(), "Person<name=san, age=21>");
}

#[test]
fn test_utf8_from() {
    let bytes = vec![0x11, 0x44, 0x33, 0x22];
    let result = String::from_utf8(bytes);

    assert_eq!(result.unwrap(), "\u{11}D3\"");

    let bytes = vec![0xff, 0x44, 0x33, 0x22];
    let result = String::from_utf8(bytes);

    // into_bytes on the error undoes the ownership done by from_utf8 and returns back the original
    // data to the user.
    assert_eq!(
        result.err().unwrap().into_bytes(),
        vec![0xff, 0x44, 0x33, 0x22]
    );
}

#[test]
fn test_cow() {
    std::env::set_var("USERNAME", "Rand");
    assert_eq!(get_user(), "Rand");

    std::env::remove_var("USERNAME");
    assert_eq!(get_user(), "Mr. Anonymouse");
}

fn get_user() -> Cow<'static, str> {
    std::env::var("USERNAME")
        .map(|v| v.into())
        .unwrap_or("Mr. Anonymouse".into())
}

#[test]
fn test_pointer_repr() {
    let original = Rc::new("rand".to_string());
    let cloned = original.clone();
    let new = Rc::new("rand".to_string());

    let mut s = String::new();

    writeln!(
        s,
        "{:<7} -> {:<14} | {:<14} | {:<14}",
        "Header", "Original", "Cloned", "New"
    )
    .unwrap();
    writeln!(
        s,
        "{:<7} -> {:<14} | {:<14} | {:<14}",
        "Values", original, cloned, new
    )
    .unwrap();
    write!(
        s,
        "{:<7} -> {:p} | {:p} | {:p}",
        "Pointer", original, cloned, new
    )
    .unwrap();

    println!("{}", s);
    assert!(s.starts_with(
        "Header  -> Original       | Cloned         | New           \
    \nValues  -> rand           | rand           | rand          \nPointer ->"
    ));
}

#[test]
fn test_keyword_args_format() {
    let s = format!(
        "{description:.<20}{quantity:2} @{cost:5.2}",
        description = "Caramel Latte",
        quantity = 2,
        cost = 4.99
    );
    assert_eq!(s, "Caramel Latte....... 2 @ 4.99");
}

use lazy_static::lazy_static;
use unicode_normalization::UnicodeNormalization;

lazy_static! {
    static ref SEMVER: Regex = Regex::new(
        r#"(?x)
                (\d+)   # major version
                \.
                (\d+)   # minor version
                \.
                (\d+)   # patch version
                (-[[:alnum:]-.]*)?  # extra info
                "#
    )
    .expect("Error parsing regex");
}

#[test]
fn test_basic_regex() {
    let version = r#"regex = "1.5.2-alpha""#;

    assert!(SEMVER.is_match(version));

    let captures = SEMVER.captures(version).unwrap();
    assert_eq!(&captures[0], "1.5.2-alpha");
    assert_eq!(&captures[1], "1");
    assert_eq!(&captures[2], "5");
    assert_eq!(&captures[3], "2");
    assert_eq!(&captures[4], "-alpha");

    assert_eq!(captures.get(0).unwrap().as_str(), "1.5.2-alpha");
    assert_eq!(captures.get(100), None);
    assert_eq!(captures.get(0).unwrap().start(), 9);
    assert_eq!(captures.get(0).unwrap().end(), 20);

    let text = "We started with 1.0.0-alpha then moved on to \
    1.0.0 which was our major release. Then we started working towards 1.1.0-beta";
    let matches = SEMVER
        .find_iter(text)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();
    assert_eq!(matches, vec!["1.0.0-alpha", "1.0.0", "1.1.0-beta"]);
}

#[test]
fn test_unicode() {
    let composed = "th\u{e9}";
    let decomposed = "the\u{301}";
    assert_ne!(composed, decomposed);
    assert!(composed > decomposed);

    assert_eq!(hash_string(composed), 6044622843544723443);
    assert_eq!(hash_string(decomposed), 10437153643056759108);

    let pho = "Phở"; // uses the 3-char representation
    assert_eq!(pho.nfd().collect::<String>(), "Pho\u{31b}\u{309}");
    assert_eq!(pho.nfc().collect::<String>(), "Ph\u{1edf}");
}

fn hash_string<S: ?Sized + Hash>(s: &S) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}
