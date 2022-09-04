use std::collections::HashMap;
use std::marker::PhantomData;
use std::mem;
use std::mem::{align_of, size_of};
use std::ops::Range;

struct Ascii(Vec<u8>);

impl Ascii {
    fn new(bytes: Vec<u8>) -> Result<Self, NotAsciiError> {
        for b in &bytes {
            if !b.is_ascii() {
                return Err(NotAsciiError(bytes));
            }
        }
        Ok(Ascii(bytes))
    }

    unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Self {
        Ascii(bytes)
    }
}

impl From<Ascii> for String {
    fn from(ascii: Ascii) -> Self {
        unsafe { String::from_utf8_unchecked(ascii.0) }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NotAsciiError(Vec<u8>);

fn option_to_raw<T>(opt: Option<&T>) -> *const T {
    match opt {
        None => std::ptr::null(),
        Some(t) => t as *const T,
    }
}

struct RefWithFlag<'a, T> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T>,
}

impl<'a, T> RefWithFlag<'a, T> {
    fn new(value: &'a T, flag: bool) -> Self {
        assert_eq!(align_of::<T>() % 2, 0);
        let ptr_and_bit = (value as *const T as usize) | (flag as usize);
        RefWithFlag {
            ptr_and_bit,
            behaves_like: PhantomData,
        }
    }

    fn get_flag(&self) -> bool {
        (self.ptr_and_bit & 1) != 0
    }

    fn get_ref(&self) -> &'a T {
        unsafe {
            let ptr = (self.ptr_and_bit & !1) as *const T;
            &*ptr
        }
    }
}

struct GapBuffer<T> {
    storage: Vec<T>,
    gap: Range<usize>,
}

impl<T> GapBuffer<T> {
    fn new() -> Self {
        GapBuffer {
            storage: vec![],
            gap: Range::default(),
        }
    }

    /// Retrieve the number of elements in this buffer
    fn size(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    /// Retrieve the capacity of this buffer
    fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    /// Retrieve the insertion point for this buffer
    fn position(&self) -> usize {
        self.gap.start
    }

    /// Retrieve the const pointer to a 'index'th element inside the buffer
    unsafe fn ptr_for(&self, index: usize) -> *const T {
        self.storage.as_ptr().offset(index as isize)
    }

    /// Retrieve the mut pointer to a 'index'th element inside the buffer
    unsafe fn mut_ptr_for(&mut self, index: usize) -> *mut T {
        self.storage.as_mut_ptr().offset(index as isize)
    }

    /// Retrieve the offset to a given 'index'th element inside the buffer taking into
    /// consideration the "gap" in the buffer. Might return an out of bounds index if provided
    /// one but would never return an index within the gap
    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    /// Retrieve index to the 'index'th element or return None if index out of bounds
    fn get(&self, index: usize) -> Option<&T> {
        let raw_index = self.index_to_raw(index);
        if raw_index >= self.capacity() {
            None
        } else {
            unsafe { self.ptr_for(raw_index).as_ref() }
        }
    }

    fn set_pos(&mut self, pos: usize) {
        if pos > self.size() {
            panic!("Out of bounds access {} for gapbuffer", pos)
        }
        let gap = self.gap.clone();
        unsafe {
            if pos > gap.start {
                let distance = pos - gap.start;
                std::ptr::copy(self.ptr_for(gap.end), self.mut_ptr_for(gap.start), distance)
            } else if pos < gap.start {
                let distance = gap.start - pos;
                std::ptr::copy(
                    self.ptr_for(pos),
                    self.mut_ptr_for(gap.end - distance),
                    distance,
                )
            }
        }
        self.gap = pos..(pos + gap.len());
    }

    fn insert(&mut self, elem: T) {
        if self.gap.is_empty() {
            self.enlarge_gap();
        }
        unsafe {
            std::ptr::write(self.mut_ptr_for(self.gap.start), elem);
        }
        self.gap.start += 1;
    }

    fn insert_iter<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        for elem in iter {
            self.insert(elem);
        }
    }

    fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }
        let element = unsafe { std::ptr::read(self.ptr_for(self.gap.end)) };
        self.gap.end += 1;
        Some(element)
    }

    fn enlarge_gap(&mut self) {
        let new_capacity = if self.capacity() == 0 {
            16
        } else {
            self.capacity() * 2
        };
        let mut new: Vec<T> = Vec::with_capacity(new_capacity);
        let last_seg_size = self.capacity() - self.gap.end;
        let new_gap = (self.gap.start)..(new_capacity - last_seg_size);

        unsafe {
            std::ptr::copy_nonoverlapping(self.ptr_for(0), new.as_mut_ptr(), self.gap.start);

            let last_seg_dst = new.as_mut_ptr().offset(new_gap.end as isize);
            std::ptr::copy_nonoverlapping(
                self.ptr_for(self.gap.end),
                new.as_mut_ptr(),
                last_seg_size,
            )
        }
        self.storage = new;
        self.gap = new_gap;
    }
}

impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for index in 0..self.gap.start {
                std::ptr::drop_in_place(self.mut_ptr_for(index));
            }
            for index in self.gap.end..self.capacity() {
                std::ptr::drop_in_place(self.mut_ptr_for(index));
            }
        }
    }
}

/// The alignment of a struct is the max of the alignment of its elements
/// The size of a struct is a multiple of its alignment value which is >= the sum of size
/// of its constituents
struct Point2d(u64, u8);

struct Point3d(u8, u8, u8);

#[repr(align(4))]
struct Point3dAligned(u8, u8, u8);

#[test]
fn test_gap_buffer() {
    let mut buffer: GapBuffer<char> = GapBuffer::new();

    assert_eq!(buffer.capacity(), 0);
    assert_eq!(buffer.size(), 0);

    buffer.insert('a');
    buffer.insert('b');
    assert_eq!(buffer.capacity(), 16);
    assert_eq!(buffer.size(), 2);

    buffer.insert_iter("cde".chars());
    assert_eq!(buffer.size(), 5);

    buffer.set_pos(2);
    assert_eq!(buffer.remove(), Some('c'));
}

#[test]
fn test_size_align() {
    let s = "abc".to_string();
    // sizeof string on stack = ptr + capacity + length = usize(8 bytes on x86_64) * 3 = 24 bytes
    assert_eq!(std::mem::size_of_val(&s), 24);

    let hm: HashMap<String, String> = HashMap::new();
    assert_eq!(mem::size_of_val(&hm), 48);

    let p = Point2d(1, 2);
    assert_eq!(mem::size_of_val(&p), 16);
    assert_eq!(mem::align_of_val(&p), 8);

    let p = Point3d(1, 2, 3);
    assert_eq!(mem::size_of_val(&p), 3);
    assert_eq!(mem::align_of_val(&p), 1);

    let p = Point3dAligned(1, 2, 3);
    assert_eq!(mem::size_of_val(&p), 4);
    assert_eq!(mem::align_of_val(&p), 4);
}

#[test]
fn test_ref_with_flag() {
    let v = vec![1, 2, 3, 4];
    let ref_wflag = RefWithFlag::new(&v, true);
    assert!(ref_wflag.get_flag());

    let ref_wflag = RefWithFlag::new(&v, false);
    assert_eq!(ref_wflag.get_flag(), false);

    let new_v = ref_wflag.get_ref();
    assert_eq!(new_v, &v);
}

#[test]
fn test_option_to_raw() {
    let num = 1;
    let empty_opt: Option<&i32> = None;
    assert_eq!(option_to_raw(empty_opt), std::ptr::null());
    assert_eq!(option_to_raw(Some(&num)), &num as *const i32);
}

#[test]
fn test_ascii() {
    let bytes: Vec<u8> = b"This is ascii!".to_vec();
    let ascii = Ascii::new(bytes).unwrap();
    assert_eq!(String::from(ascii), "This is ascii!".to_string());

    let bytes: Vec<u8> = "नमस्ते".to_string().into_bytes();
    let ascii = Ascii::new(bytes);
    assert!(ascii.is_err());
}

#[test]
fn test_unsafe_ascii() {
    // Imagine that this vector is the result of some complicated process
    // that we expected to produce ASCII. Something went wrong!
    let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];

    let ascii = unsafe {
        // This unsafe function's contract is violated
        // when `bytes` holds non-ASCII bytes.
        Ascii::from_bytes_unchecked(bytes)
    };

    let bogus: String = ascii.into();

    // `bogus` now holds ill-formed UTF-8. Parsing its first character produces
    // a `char` that is not a valid Unicode code point. That's undefined
    // behavior, so the language doesn't say how this assertion should behave.
    assert_eq!(bogus.chars().next().unwrap() as u32, 0x1fffff);
}
