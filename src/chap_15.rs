use crate::chap_15::BinaryTree::NonEmpty;
use num::{Complex, Zero};
use rand::random;
use std::cmp::{max, min};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::BufRead;
use std::iter::{from_fn, once, repeat, successors, zip, Cycle, Peekable};
use std::path::{Iter, Path};
use std::str::FromStr;

#[test]
fn test_custom_tree_iter() {
    let mut tree = BinaryTree::Empty;
    tree.add(6);
    tree.add(3);
    tree.add(9);
    tree.add(8);
    tree.add(4);

    let nodes = tree.iter().cloned().collect::<Vec<_>>();
    assert_eq!(nodes, vec![3, 4, 6, 8, 9]);
}

struct TreeIter<'a, T> {
    unvisited: Vec<&'a TreeNode<T>>,
}

impl<'a, T> TreeIter<'a, T> {
    fn add_candidate(&mut self, mut tree: &'a BinaryTree<T>) {
        while let NonEmpty(ref node) = *tree {
            self.unvisited.push(node);
            tree = &node.left;
        }
    }
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.unvisited.pop()?;
        self.add_candidate(&node.right);
        Some(&node.value)
    }
}

enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

impl<T: Ord> BinaryTree<T> {
    fn iter(&self) -> TreeIter<T> {
        let mut iter = TreeIter {
            unvisited: Vec::new(),
        };
        iter.add_candidate(self);
        iter
    }

    fn add(&mut self, element: T) {
        match *self {
            BinaryTree::Empty => {
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    value: element,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                }))
            }
            BinaryTree::NonEmpty(ref mut existing) => {
                if element < existing.value {
                    existing.left.add(element);
                } else {
                    existing.right.add(element);
                }
            }
        }
    }
}

struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

#[test]
fn test_custom_iter() {
    let r = I32Range { start: 1, end: 4 };
    let v = r.collect::<Vec<_>>();
    assert_eq!(v, vec![1, 2, 3, 4]);
}

struct I32Range {
    start: i32,
    end: i32,
}

impl Iterator for I32Range {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }
        let res = Some(self.start);
        self.start += 1;
        res
    }
}

#[test]
fn test_extend() {
    let mut s = "HI ".to_string();
    s.extend("HELLO".chars());
    assert_eq!(s, "HI HELLO");
}

#[test]
fn test_from_iter() {
    let v = vec![1, 2, 3];

    let vec = v.iter().cloned().collect::<Vec<_>>();
    assert_eq!(vec, vec![1, 2, 3]);

    let hs = v.iter().cloned().collect::<HashSet<_>>();
    assert_eq!(hs, HashSet::from([1, 2, 3]));

    let ts = v.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(ts, BTreeSet::from([1, 2, 3]));

    let mut iter = (0..).zip(v);
    let hm = iter.by_ref().collect::<HashMap<_, _>>();
    assert_eq!(hm, HashMap::from([(0, 1), (1, 2), (2, 3)]));

    let tm = iter.collect::<BTreeMap<_, _>>();
    assert_eq!(hm, HashMap::from([(0, 1), (1, 2), (2, 3)]));
}

#[test]
fn test_find() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(None, populations.iter().find(|(_, &v)| v > 1_000_000));
    assert_eq!(
        Some((&"Portland", &583_776)),
        populations.iter().find(|(_, &v)| v > 500_000)
    );

    let opt = populations
        .iter()
        .find_map(|(&name, &pop)| if pop < 10 { Some(name) } else { None });
    assert_eq!(opt, Some("Greenhorn"));
}

#[test]
fn test_last() {
    let squares = (1..11).map(|n| n * n);
    assert_eq!(squares.last(), Some(100));

    let v = vec!["1".to_string(), "2".to_string(), "3".to_string()];
    let squares = v.iter().map(|s| {
        let mut t = String::new();
        t.push_str(s);
        t.push_str("_100");
        t
    });
    assert_eq!(squares.last(), Some("3_100".to_string()));
}

#[test]
fn test_nth() {
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter();

    assert_eq!(iter.nth(3), Some(&4));
    assert_eq!(iter.nth(0), Some(&5));
    assert_eq!(iter.nth(13), None);

    let mut iter = v.iter();
    assert_eq!(iter.nth_back(2), Some(&3));
    assert_eq!(iter.nth_back(2), None);
}

#[test]
fn test_try_folds() {
    let values = vec!["1", "2", "3", "abcd", "100"];
    let res = values
        .iter()
        .try_fold(0, |a, v| -> Result<i32, Box<dyn Error>> {
            Ok(a + v.parse::<i32>()?)
        });
    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "invalid digit found in string"
    );
}

#[test]
fn test_folds() {
    let v = vec![1, 2, 3, 4, 5];

    assert_eq!(v.iter().fold(0, |a, v| a + v), 15);
    assert_eq!(v.iter().fold(1, |a, v| a * v), 120);
    assert_eq!(v.iter().fold(i32::MIN, |a, b| max(a, *b)), 5);

    let v = vec!["a", "b", "c"];
    assert_eq!(v.iter().fold(String::new(), |s, w| s + w), "abc");
    assert_eq!(v.iter().rfold(String::new(), |s, w| s + w), "cba");
}

#[test]
fn test_any_all() {
    let s = "Hello guys!";

    assert!(s.chars().any(char::is_uppercase));
    assert_eq!(s.chars().all(char::is_alphabetic), false);
}

#[test]
fn test_lt_gt_eq() {
    let s1 = "Wheel Of Time";
    let s2 = "Wheel    Of    Time";
    let s3 = "Wheel Of Destiny";

    assert!(s1 > s2);
    assert!(s1.split_ascii_whitespace().eq(s2.split_ascii_whitespace()));

    assert!(s2.split_ascii_whitespace().gt(s3.split_ascii_whitespace()));
}

#[test]
#[should_panic]
fn test_min_max_by() {
    let v = vec![1.2, 23.2, 1232.2, -12.1];

    assert_eq!(
        v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()),
        Some(&-12.1)
    );
    assert_eq!(
        v.iter().max_by(|a, b| a.partial_cmp(b).unwrap()),
        Some(&1231.2)
    );

    // Panics
    let v = vec![1.2, 23.2, 1232.2, -12.1, f64::NAN];

    assert_eq!(
        v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()),
        Some(&-12.1)
    );
}

#[test]
fn test_min_max() {
    let v = vec![1, 4, 93, 23, 4, 1231, 12, 0, 23, -23];

    assert_eq!(v.iter().min(), Some(&-23));
    assert_eq!(v.iter().max(), Some(&1231));
}

#[test]
fn test_sum_product() {
    assert_eq!((1..100).into_iter().sum::<u64>(), 4950);

    assert_eq!((1..20).into_iter().product::<u64>(), 121645100408832000);
}

#[test]
fn test_fizzbuzz() {
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fb_zip = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fb_zip).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    let few = fizz_buzz.take(16).collect::<Vec<_>>();
    let few = few.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    assert_eq!(
        few,
        vec![
            "1", "2", "fizz", "4", "buzz", "fizz", "7", "8", "fizz", "buzz", "11", "fizz", "13",
            "14", "fizzbuzz", "16"
        ]
    );
}

#[test]
fn test_cycle() {
    let v = vec!["yes", "no"];
    let mut vicious_cycle = v.iter().cycle();
    assert_eq!(vicious_cycle.next(), Some(&"yes"));
    assert_eq!(vicious_cycle.next(), Some(&"no"));
    assert_eq!(vicious_cycle.next(), Some(&"yes"));
    assert_eq!(vicious_cycle.next(), Some(&"no"));
}

#[test]
fn test_cloned() {
    let v = vec![1, 2, 3, 4];

    assert_eq!(v.iter().next(), Some(&1));
    assert_eq!(v.iter().cloned().next(), Some(1));
}

#[test]
fn test_byref() {
    let text = "From: jim.butcher@fantasy.com\r\n\
    To: sanjay@fantasy.com\r\n\
    Subject: My new book!\r\n\
    \r\n\
    Hey, have you seen my new book which came out last week?";
    let mut iter = text.lines();

    let headers = iter
        .by_ref()
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        headers,
        vec![
            "From: jim.butcher@fantasy.com",
            "To: sanjay@fantasy.com",
            "Subject: My new book!"
        ]
    );

    let body = iter
        .by_ref()
        .skip_while(|s| s.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        body,
        vec!["Hey, have you seen my new book which came out last week?"]
    );
}

#[test]
fn test_zip() {
    let pairs = (0..).zip(vec!["a", "b", "c"]).collect::<Vec<_>>();
    assert_eq!(pairs, vec![(0, "a"), (1, "b"), (2, "c")]);

    let numbers = vec![1, 2, 3];
    let pairs = repeat("take").zip(numbers).collect::<Vec<_>>();
    assert_eq!(pairs, vec![("take", 1), ("take", 2), ("take", 3)]);
}

#[test]
fn test_inspect() {
    let s = "große";
    let supper: String = s
        .chars()
        .inspect(|c| println!("Before -> {:?}", c))
        .flat_map(|c| c.to_uppercase())
        .inspect(|c| println!("After -> {:?}", c))
        .collect();
    assert_eq!(supper, "GROSSE");
}

#[test]
fn test_rev() {
    let parts = vec!["head", "shoulders", "knees", "toes"];
    let mut iter = parts.iter();

    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"toes"));
    assert_eq!(iter.next(), Some(&"shoulders"));
    assert_eq!(iter.next_back(), Some(&"knees"));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);

    let lengths = parts.iter().map(|s| s.len()).rev().collect::<Vec<_>>();
    assert_eq!(lengths, vec![4, 5, 9, 4]);
}

#[test]
fn test_fuse() {
    let mut flaky = ItsFlaky(true);
    assert_eq!(flaky.next(), Some("Last item -- I promise!"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), Some("Last item -- I promise!"));

    let mut flaky = ItsFlaky(true).fuse();
    assert_eq!(flaky.next(), Some("Last item -- I promise!"));
    assert_eq!(flaky.next(), None);
    assert_eq!(flaky.next(), None);
}

struct ItsFlaky(bool);

impl Iterator for ItsFlaky {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 {
            self.0 = false;
            Some("Last item -- I promise!")
        } else {
            self.0 = true;
            None
        }
    }
}

#[test]
fn test_peek() {
    let mut chars = "1234,5678".chars().peekable();

    assert_eq!(parse_number(&mut chars), 1234);
    assert_eq!(chars.next(), Some(','));
    assert_eq!(parse_number(&mut chars), 5678);
    assert_eq!(chars.next(), None);
}

fn parse_number<I>(tokens: &mut Peekable<I>) -> u32
where
    I: Iterator<Item = char>,
{
    let mut n = 0_u32;
    while let Some(c) = tokens.peek() {
        if c.is_numeric() {
            n = n * 10 + c.to_digit(10).unwrap();
            tokens.next();
        } else {
            return n;
        }
    }
    n
}

#[test]
fn test_skip() {
    let text = "From: jim.butcher@fantasy.com\r\n\
    To: sanjay@fantasy.com\r\n\
    Subject: My new book!\r\n\
    \r\n\
    Hey, have you seen my new book which came out last week?";
    let body = text
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .collect::<Vec<_>>();
    assert_eq!(
        body,
        vec!["Hey, have you seen my new book which came out last week?"]
    )
}

#[test]
fn test_take() {
    let text = "From: jim.butcher@fantasy.com\r\n\
    To: sanjay@fantasy.com\r\n\
    Subject: My new book!\r\n\
    \r\n\
    Hey, have you seen my new book which came out last week?";
    let headers = text
        .lines()
        .take_while(|s| !s.is_empty())
        .collect::<Vec<_>>();
    assert_eq!(
        headers,
        vec![
            "From: jim.butcher@fantasy.com",
            "To: sanjay@fantasy.com",
            "Subject: My new book!"
        ]
    )
}

#[test]
fn test_flatten() {
    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor Park", "Forest Park"]);
    parks.insert("Kyoto", vec!["Tadasu-no-Mori Forest", "Maruyama Koen"]);
    parks.insert("Nashville", vec!["Percy Warner Park", "Dragon Park"]);

    let all_parks = parks.values().flatten().cloned().collect::<Vec<_>>();
    assert_eq!(
        all_parks,
        vec![
            "Tadasu-no-Mori Forest",
            "Maruyama Koen",
            "Percy Warner Park",
            "Dragon Park",
            "Mt. Tabor Park",
            "Forest Park"
        ]
    );

    let all_values = vec![Some("one"), None, None, Some("two")];
    let actual_values = all_values.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(actual_values, vec!["one", "two"]);
}

#[test]
fn test_flat_map() {
    let mut cities = HashMap::new();
    cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    cities.insert("India", vec!["Mumbai", "Bangalore", "Delhi"]);
    cities.insert("United Kingdom", vec!["London", "Birmingham", "Glasgow"]);
    cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = vec!["India", "Kenya"];
    let selected_cities = countries
        .into_iter()
        // doesn't work without .clone() below, why?
        .flat_map(|v| (&cities[v]).clone())
        .collect::<Vec<_>>();
    assert_eq!(
        selected_cities,
        vec!["Mumbai", "Bangalore", "Delhi", "Nairobi", "Mombasa"]
    );
}

#[test]
fn test_filter_map() {
    let text = "Ready Player One   \n Wheel Of Time  \nHarry Potter\nEarthsea Cycle";
    let novels = text.lines().map(|s| s.trim()).collect::<Vec<_>>();
    assert_eq!(
        novels,
        vec![
            "Ready Player One",
            "Wheel Of Time",
            "Harry Potter",
            "Earthsea Cycle"
        ]
    );

    let ends_with_e = text
        .lines()
        .map(str::trim)
        .filter(|s| !s.ends_with('e'))
        .collect::<Vec<_>>();
    assert_eq!(ends_with_e, vec!["Harry Potter"]);

    let text = "abba\n2 \n dabba \n 8";
    let numbers = text
        .split_ascii_whitespace()
        .filter_map(|w| usize::from_str(w).ok())
        .collect::<Vec<_>>();
    assert_eq!(numbers, vec![2, 8]);

    let text = "some\ndata";
    let chars = text.lines().flat_map(|w| w.chars()).collect::<Vec<_>>();
    assert_eq!(chars, vec!['s', 'o', 'm', 'e', 'd', 'a', 't', 'a']);
}

#[test]
fn test_drain() {
    let mut original = "WheelOfTime".to_string();
    let mut pt1 = String::from_iter(original.drain(1..5));
    pt1.push_str(String::from_iter(original.drain(2..3)).as_str());
    pt1.push_str(String::from_iter(original.drain(3..)).as_str());

    assert_eq!(original, "WOT");
    assert_eq!(pt1, "heelfime");
}

#[test]
fn test_fib() {
    let mut iter = gen_fib();

    assert_eq!(
        iter.take(10).collect::<Vec<_>>(),
        vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    );
}

fn gen_fib() -> impl Iterator<Item = usize> {
    let mut state = (0, 1);
    from_fn(move || {
        let ans = Some(state.0);
        state = (state.1, state.0 + state.1);
        ans
    })
}

#[test]
fn test_successors() {
    let c1 = Complex::new(0.05, 0.02);
    assert_eq!(complex_successors(c1, 2000), None);
}

fn complex_successors(c: Complex<f64>, limit: usize) -> Option<usize> {
    let zero = Complex::zero();
    std::iter::successors(Some(zero), |&z: &Complex<f64>| Some(z * z + c))
        .take(limit)
        .enumerate()
        .find(|(_, z)| z.norm_sqr() > 4.0)
        .map(|(i, _)| i)
}

#[test]
fn test_randoms() {
    let v = gen_randoms(100);

    assert_eq!(v.len(), 100);
    assert!(v.first().unwrap() >= &0.0);
}

fn gen_randoms(limit: usize) -> Vec<f64> {
    std::iter::from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(limit)
        .collect()
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
