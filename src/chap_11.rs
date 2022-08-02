extern crate core;

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{stdout, BufWriter, Write};
use std::ops::{Add, Mul};

#[test]
fn test_dot_product() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    assert_eq!(dot(&v1, &v2), 32);

    let v1: Vec<i64> = vec![1, 2, 3];
    let v2: Vec<i64> = vec![4, 5, 6];
    assert_eq!(dot(v1.as_slice(), v2.as_slice()), 32);
}

fn dot<T>(v1: &[T], v2: &[T]) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    let mut sum = T::default();
    for idx in 0..v1.len() {
        sum = sum + (v1[idx] * v2[idx]);
    }
    sum
}

#[test]
fn test_add() {
    let out = fib::<f64>(4);
    assert_eq!(out, 3.0);
}

trait MyFloat {
    const ZERO: Self;
    const ONE: Self;
}

impl MyFloat for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

fn fib<T: MyFloat + Add<Output = T>>(n: usize) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        _ => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

#[test]
fn test_custom_mul() {
    let a1 = AppleBasket { count: 15 };
    let a2 = AppleBasket { count: 4 };

    assert_eq!(a1 * a2, AppleBasket { count: 60 });

    let a1 = AppleBasket { count: 15 };
    assert_eq!(a1 * 4, AppleBasket { count: 60 });

    let a1 = AppleBasket { count: 15 };
    assert_eq!(4 * a1, AppleBasket { count: 60 });
}

fn open(path: &str) -> io::Result<Box<dyn Write>> {
    match path {
        "-" => Ok(Box::new(BufWriter::new(stdout()))),
        _ => Ok(Box::new(BufWriter::new(File::create(path)?))),
    }
}

impl Mul for AppleBasket {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        AppleBasket {
            count: self.count * rhs.count,
        }
    }
}

impl Mul<usize> for AppleBasket {
    type Output = AppleBasket;

    fn mul(self, rhs: usize) -> Self::Output {
        AppleBasket {
            count: self.count * rhs,
        }
    }
}

impl Mul<AppleBasket> for usize {
    type Output = AppleBasket;

    fn mul(self, rhs: AppleBasket) -> Self::Output {
        AppleBasket {
            count: self * rhs.count,
        }
    }
}

#[derive(PartialEq, Debug)]
struct AppleBasket {
    count: usize,
}

#[test]
fn test_dump() {
    let v = vec![Num(1), Num(2)];
    dump(v.iter());
}

#[derive(Debug)]
struct Num(i32);

#[test]
fn test_stringset() {
    let mut hs = StringSetHash::new();
    hs.add("HI").add("HELLO").add("WOW").add("NOICE");
    let u = find_unknown(&["HELLO", "NOICE", "whoa", "y u not?"], &hs);
    println!("Unknown is {:?}", u);
}

fn dump<I>(iter: I)
where
    I: Iterator,
    I::Item: Debug,
{
    for (idx, val) in iter.enumerate() {
        println!("Index={} and value={:?}", idx, val);
    }
}

fn find_unknown<'a, S: StringSet<'a>>(words: &[&'a str], set: &S) -> S {
    let mut u = S::new();
    for w in words {
        if !set.contains(w) {
            u.add(*w);
        }
    }
    u
}

#[derive(Debug)]
struct StringSetHash<'a> {
    names: HashSet<&'a str>,
}

impl<'a> StringSet<'a> for StringSetHash<'a> {
    fn new() -> Self {
        StringSetHash {
            names: HashSet::new(),
        }
    }

    fn add(&mut self, name: &'a str) -> &mut Self {
        self.names.insert(name);
        self
    }

    fn contains(&self, name: &str) -> bool {
        self.names.contains(name)
    }
}

trait StringSet<'a> {
    /// Create a new string set
    fn new() -> Self;

    /// Add a new item to the existing string set
    fn add(&mut self, name: &'a str) -> &mut Self;

    /// Check if teh string set contains a given string
    fn contains(&self, name: &str) -> bool;
}

fn write<T: Write>(data: &str, writer: &mut T) -> io::Result<()> {
    writer.write_all(data.as_bytes())
}

#[derive(Debug)]
struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
