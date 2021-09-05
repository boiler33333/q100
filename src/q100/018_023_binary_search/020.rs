use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };

pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less => left = mid + 1,
        Equal | Greater => right = mid,
      }
    }
    left
  }

  fn upper_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less | Equal => left = mid + 1,
        Greater => right = mid,
      }
    }
    left
  }
}

fn main() {
  input! {
    n: usize,
    mut a: [usize; n],
    mut b: [usize; n],
    mut c: [usize; n],
  }
  a.sort();
  b.sort();
  c.sort();
  let mut ans = 0;
  for j in &b {
    let i = a.lower_bound(j);
    let k = c.upper_bound(j);
    ans += i * (n - k);
  }
  println!("{}", ans);
}