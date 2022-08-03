use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::path::Path;

#[test]
fn test_randoms() {
    let v = gen_randoms(100);

    assert_eq!(v.len(), 100);
    assert!(v.first().unwrap() >= &0.0);
}

fn gen_randoms(limit: usize) -> Vec<f64> {
    std::iter::from_fn(|| {

    })
    let v: Vec<f64> = Vec::new();
    v
}

#[test]
fn test_generic_dump() {
    dump(vec!["hi", "hello", "bye"]);
}

fn dump<T, I>(iterable: T)
where
    T: IntoIterator<Item = I>,
    I: Debug,
{
    for v in iterable {
        println!("Dumping data --> {:?}", v);
    }
}

#[test]
fn test_btreeset_iter() {
    let mut bts = BTreeSet::new();
    bts.insert("Wheel Of Time");
    bts.insert("Game Of Thrones");

    // iterator over item references inside Btreeset
    let mut iter = (&bts).into_iter();
    assert_eq!(iter.next(), Some(&"Game Of Thrones"));
    assert_eq!(iter.next(), Some(&"Wheel Of Time"));
    assert_eq!(iter.next(), None);

    // mutable iterator over item references
    let mut v = vec!["abba".to_string(), "dabba".to_string(), "jabba".to_string()];
    let iter = (&mut v).into_iter();
    for s in iter {
        s.push_str(" do");
    }

    // By value into_iter basically moves elements into the iterator
    let mut iter = v.into_iter();
    assert_eq!(iter.next(), Some("abba do".to_string()));
    assert_eq!(iter.next(), Some("dabba do".to_string()));
    assert_eq!(iter.next(), Some("jabba do".to_string()));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_path_iter() {
    let p = Path::new("/tmp/sanjayts/my documents");
    let mut iter = p.iter();

    assert_eq!(iter.next(), Some(OsStr::new("/")));
    assert_eq!(iter.next(), Some(OsStr::new("tmp")));
    assert_eq!(iter.next(), Some(OsStr::new("sanjayts")));
    assert_eq!(iter.next(), Some(OsStr::new("my documents")));
}

#[test]
fn test_vec_into_iter() {
    let v = vec!["abba", "dabba", "jabba"];
    let mut iter = (&v).into_iter();
    while let Some(v) = iter.next() {
        println!("Iter element is {}", v);
    }
}

#[test]
fn test_triangle_num() {
    assert_eq!(6, triangle_number(3));
    assert_eq!(6, triangle_number_fold(3));
}

fn triangle_number_fold(n: u32) -> u32 {
    (1..=n).fold(0, |sum, n| sum + n)
}

fn triangle_number(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}
