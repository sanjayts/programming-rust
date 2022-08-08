use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashSet, VecDeque};

#[test]
fn test_first() {
    let v = vec![1, 2, 3, 4];
    assert_eq!(v.first(), Some(&1));
}

#[test]
fn test_last_mut() {
    let mut slice = [1, 2, 3, 4];
    let last = slice.last_mut().unwrap();
    assert_eq!(*last, 4);

    *last = 100;
    assert_eq!(slice, [1, 2, 3, 100]);
}

#[test]
fn test_to_vec() {
    let v = [1, 2, 3, 4, 5];

    assert_eq!(v[1..2].to_vec(), vec![2]);
}

#[test]
fn test_resize() {
    let mut v = vec![1, 2];
    v.resize_with(4, || 100);
    assert_eq!(v, vec![1, 2, 100, 100]);
}

#[test]
fn test_retain() {
    let mut byte_vec = b"Misssssssissippi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|e| seen.insert(*e));

    assert_eq!(byte_vec, b"Misp".to_vec());
}

#[test]
fn test_join() {
    let v = [[1, 2], [3, 4], [5, 6]];
    assert_eq!(v.concat(), [1, 2, 3, 4, 5, 6]);

    assert_eq!(v.join(&0), [1, 2, 0, 3, 4, 0, 5, 6]);
}

#[test]
fn test_chunks() {
    let mut v = [1, 2, 3, 4, 5, 6];
    let mid = v.len() / 2;
    let (front, back) = (&v[..mid], &v[mid..]);

    assert_eq!(front, &[1, 2, 3]);
    assert_eq!(back, &[4, 5, 6]);

    let all = &mut v[..mid];
    all.swap(0, 2);
    let mut chunks = all.chunks(1);

    assert_eq!(chunks.next(), Some([3].as_ref()));
    assert_eq!(chunks.next(), Some([2].as_ref()));
    assert_eq!(chunks.next(), Some([1].as_ref()));
    assert_eq!(chunks.next(), None);
}

#[test]
fn test_sort_key() {
    let s1 = Student {
        first_name: "Sanjay".to_string(),
        last_name: "Sharma".to_string(),
    };
    let s2 = Student {
        first_name: "Ankit".to_string(),
        last_name: "Phadia".to_string(),
    };
    let s3 = Student {
        first_name: "Sanjay".to_string(),
        last_name: "Kumar".to_string(),
    };
    let s4 = Student {
        first_name: "Ankit".to_string(),
        last_name: "Jain".to_string(),
    };
    let s5 = Student {
        first_name: "Chirag".to_string(),
        last_name: "Joshi".to_string(),
    };
    let (s6, s7, s8, s9) = (s1.clone(), s2.clone(), s3.clone(), s4.clone());
    let first_expected = s4.clone();
    let last_expected = s1.clone();
    let mut students = [s1, s2, s3, s4, s5, s6, s7, s8, s9];

    students.sort_unstable_by(|a, b| {
        // println!("Calling cmp for {:?} and {:?}", a, b);
        let t1 = (&a.first_name, &a.last_name);
        let t2 = (&b.first_name, &b.last_name);
        t1.cmp(&t2)
    });

    assert_eq!(students.first().unwrap(), &first_expected);
    assert_eq!(students.last().unwrap(), &last_expected);
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Student {
    first_name: String,
    last_name: String,
}

#[test]
fn test_deque() {
    let mut dq = VecDeque::new();

    dq.push_front(1);
    dq.push_front(2);
    dq.push_front(3);

    assert_eq!(dq.pop_back(), Some(1));
    assert_eq!(dq.pop_back(), Some(2));
    assert_eq!(dq.pop_back(), Some(3));
}

#[test]
fn test_min_heap() {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(6));
    heap.push(Reverse(2));
    heap.push(Reverse(1));
    heap.push(Reverse(9));

    let mut v = Vec::new();
    while let Some(r) = heap.pop() {
        v.push(r.0);
    }
    assert_eq!(v, vec![1, 2, 6, 9]);
}

#[test]
fn test_hash_btree_maps() {
    let mut btm = BTreeMap::new();
    btm.insert(1, "HI");
    btm.insert(2, "HO");
    btm.insert(3, "EK");
    btm.insert(4, "AI");

    let new_btm = btm.split_off(&3);
    assert_eq!(btm.keys().collect::<Vec<&i32>>(), vec![&1, &2]);
    assert_eq!(new_btm.keys().collect::<Vec<&i32>>(), vec![&3, &4]);
}

#[test]
fn test_sets() {
    let lucky_numbers = HashSet::from([1, 5, 9]);
    let all_dates = (1..11).collect::<HashSet<i32>>();

    assert_eq!(
        &all_dates - &lucky_numbers,
        HashSet::from([2, 3, 4, 6, 7, 8, 10])
    );

    assert_eq!(&all_dates & &lucky_numbers, HashSet::from([1, 5, 9]));
    assert_eq!(&lucky_numbers | &all_dates, all_dates);

    assert!(&all_dates.is_superset(&lucky_numbers));
    assert!(&lucky_numbers.is_subset(&all_dates));
}
