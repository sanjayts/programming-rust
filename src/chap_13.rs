use std::borrow::Cow;
use std::collections::HashSet;

use std::error::Error;
use std::fmt::Display;

use std::net::Ipv4Addr;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[test]
fn test_cow() {
    let msg = describe(&CliError::InvalidSize(23));
    assert_eq!(msg, "Invalid size 23");

    let mut errors: Vec<String> = vec![];
    let msg = describe(&CliError::NonExistentFile("a.txt"));
    errors.push(msg.into_owned());
    assert_eq!(errors, vec!["Non-existent file a.txt".to_string()]);
}

fn describe(e: &CliError) -> Cow<'static, str> {
    match *e {
        CliError::TooFewArguments => "Too Few Arguments".into(),
        CliError::TooManyArguments => "Too many arguments".into(),
        CliError::InvalidSize(sz) => format!("Invalid size {}", sz).into(),
        CliError::NonExistentFile(file) => format!("Non-existent file {}", file).into(),
    }
}

enum CliError<'a> {
    TooFewArguments,
    TooManyArguments,
    InvalidSize(usize),
    NonExistentFile(&'a str),
}

#[test]
fn test_from_into() {
    let s = "HI";
    let u: String = From::from(s);
    assert_eq!(s, u);

    ping(Ipv4Addr::from_str("128.0.0.1").unwrap());
    ping([127, 0, 0, 1]);
    ping(127_0_0_1_u32);

    let i1 = parse_i32(&[57, 56]);
    assert_eq!(i1.unwrap(), 98);
    let i2 = parse_i32(&[57, 56, 57, 56, 57, 56, 57, 56, 57, 56, 57, 56, 57, 56, 57]);
    assert_eq!(
        i2.unwrap_err().to_string(),
        "number too large to fit in target type"
    );
}

type GenericError = Box<dyn Error + Send + Sync + 'static>;

type GenericResult<T> = Result<T, GenericError>;

fn parse_i32(bytes: &[u8]) -> GenericResult<i32> {
    Ok(std::str::from_utf8(bytes)?.parse::<i32>()?)
}

fn ping<A>(ip_addr: A)
where
    A: Into<Ipv4Addr>,
{
    let ip_addr = ip_addr.into();
    println!("Will ping address {:?}", ip_addr);
}

#[test]
fn test_asref() {
    let take_str = |s: &str| println!("{}", s);
    let take_bytes = |barr: &[u8]| println!("{:?}", barr);

    let s = "DOOM".to_string();
    let mb = MyBuffer {
        bytes: vec![1, 2, 3, 4],
    };

    take_str(s.as_ref());
    take_bytes(s.as_ref());
    take_bytes(mb.as_ref());
}

struct MyBuffer {
    bytes: Vec<u8>,
}

impl AsRef<Vec<u8>> for MyBuffer {
    fn as_ref(&self) -> &Vec<u8> {
        &self.bytes
    }
}

#[test]
fn test_default() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (p1, p2): (HashSet<i32>, HashSet<i32>) = numbers.iter().partition(|&n| n & (n - 1) == 0);
    assert_eq!(p1, HashSet::from([1, 2, 4, 8]));
    assert_eq!(p2, HashSet::from([3, 5, 6, 7]));

    let numbers = 1..16;
    let (p1, _): (Accumulator<i32>, Accumulator<i32>) = numbers.partition(|n| n % 3 == 0);
    assert_eq!(
        p1,
        Accumulator {
            elems: vec![3, 6, 9, 12, 15]
        }
    );
}

#[derive(PartialEq, Debug, Default)]
struct Accumulator<T> {
    elems: Vec<T>,
}

impl<T> Extend<T> for Accumulator<T> {
    fn extend<A: IntoIterator<Item = T>>(&mut self, iter: A) {
        self.elems.extend(iter)
    }
}

#[test]
fn test_deref() {
    let mut sel = Selector {
        elements: vec!['a', 'b', 'c', 'd'],
        current: 2,
    };
    assert_eq!(*sel, 'c');
    assert!(sel.is_alphabetic());
    *sel = 'z';
    assert_eq!(sel.elements, vec!['a', 'b', 'z', 'd']);

    let sel = Selector {
        elements: vec!["good", "bad", "ugly"],
        current: 2,
    };
    print_it(*sel);
    println!("Selector values are: {:?}", sel.elements);
}

fn print_it<T: Display>(v: T) {
    println!("Value is {}", v);
}

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_sized() {
    let boxed = RcBox {
        count: 1,
        value: "SOMETHING".to_owned(),
    };
    let display_box: &RcBox<dyn Display> = &boxed;
    show_box(display_box);
}

fn show_box(boxed: &RcBox<dyn Display>) {
    println!("box value is: {}", &boxed.value);
}

struct RcBox<T: ?Sized> {
    count: usize,
    value: T,
}

#[test]
fn test_drop() {
    let mut a = Appelation {
        name: "Rand Al Thor".to_owned(),
        nicks: vec!["Dragon Reborn".to_owned(), "Chosen One".to_string()],
    };
    a = Appelation {
        name: "Moraine".to_owned(),
        nicks: vec![],
    };
    println!("Done with the code");
}

struct Appelation {
    name: String,
    nicks: Vec<String>,
}

impl Drop for Appelation {
    fn drop(&mut self) {
        print!("Drop called for {}", self.name);
        if !self.nicks.is_empty() {
            print!(" -- AKA ({})", self.nicks.join(", "));
        }
        println!();
    }
}
