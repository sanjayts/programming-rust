use std::cmp::{Ordering, Reverse};
use std::ops::{Add, AddAssign, Index, IndexMut, Neg};

#[test]
fn test_index() {
    let mut img: Image<u8> = Image::new(5, 5);

    let mut_slice = &mut img[2];
    mut_slice[1] = 1;
    mut_slice[3] = 9;

    let slice = &img[2];
    assert_eq!([0, 1, 0, 9, 0], slice);
}

struct Image<P> {
    width: usize,
    pixels: Vec<P>,
}

impl<P> Image<P>
where
    P: Copy + Default,
{
    fn new(height: usize, width: usize) -> Self {
        Image {
            width,
            pixels: vec![P::default(); height * width],
        }
    }
}

impl<P> Index<usize> for Image<P> {
    type Output = [P];

    fn index(&self, index: usize) -> &Self::Output {
        let start_pos = index * self.width;
        let end_pos = start_pos + self.width;
        &self.pixels[start_pos..end_pos]
    }
}

impl<P> IndexMut<usize> for Image<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start_pos = index * self.width;
        let end_pos = start_pos + self.width;
        &mut self.pixels[start_pos..end_pos]
    }
}

#[test]
fn test_interval() {
    let t1 = Interval::new(5, 10);
    let t2 = Interval::new(6, 15);
    let t3 = Interval::new(1, 4);
    let t4 = Interval::new(25, 30);

    assert!(t1 > t3);
    assert!(t1 >= t3);
    assert!(t2 <= t4);
    assert!(t2 <= t2);

    let mut v = vec![t1, t2, t3, t4];
    v.sort_by_key(|it| it.lower);
    assert_eq!(v, vec![t3, t1, t2, t4]);

    v.sort_unstable_by_key(|it| Reverse(it.lower));
    assert_eq!(v, vec![t4, t2, t1, t3]);
}

#[derive(PartialEq, Debug, Clone, Copy)]
struct Interval<T> {
    lower: T,
    upper: T,
}

impl<T> Interval<T> {
    fn new(lower: T, upper: T) -> Interval<T> {
        Interval { lower, upper }
    }
}

impl<T: PartialOrd> PartialOrd for Interval<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.lower > other.upper {
            Some(Ordering::Greater)
        } else if self.upper < other.lower {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

#[test]
fn test_complex() {
    let c1 = Complex { re: 1, im: 4 };
    let c2 = Complex { re: 4, im: 2 };
    assert_eq!(c1 + c2, Complex { re: 5, im: 6 });

    let c1 = Complex { re: 1.5, im: 4.4 };
    let c2 = Complex { re: 4.2, im: 2.1 };
    assert_eq!(c1 + c2, Complex { re: 5.7, im: 6.5 });

    let c1 = Complex { re: 1.5, im: 4.4 };
    assert_eq!(-c1, Complex { re: -1.5, im: -4.4 });

    let mut c1 = Complex { re: 1.5, im: 4.4 };
    let c2 = Complex { re: 4.2, im: 2.1 };
    c1 += c2;
    assert_eq!(c1, Complex { re: 5.7, im: 6.5 });
}

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T: PartialEq> PartialEq for Complex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: AddAssign> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.im += rhs.im;
        self.re += rhs.re;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}
