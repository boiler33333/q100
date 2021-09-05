use proconio::input;
use std::cmp::max;
use std::cmp::Ordering::{ Less, Equal, Greater };

pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
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
}

fn main() {
  input! {
    n: usize,
    m: usize,
    mut p: [usize; n],
  }
  p.push(0);
  let mut s = Vec::new();
  for p1 in &p {
    for p2 in &p {
      s.push(p1 + p2);
    }
  }
  s.sort();
  let mut ans = 0;
  for &a in &s {
    if m < a {
      break;
    }
    let b = m - a;
    let i = s.lower_bound(&b) - 1;
    ans = max(a + s[i], ans);
  }
  println!("{}", ans);
}