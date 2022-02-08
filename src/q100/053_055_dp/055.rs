use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };
use std::usize::MAX;

trait BinarySearch<T> {
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn upper_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Equal | Less => left = mid + 1,
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
  }
  a.reverse();
  let mut dp = vec![MAX; n+1];
  for v in a {
    let i = dp.upper_bound(&v);
    dp[i] = v;
  }
  let ans = dp.iter().position(|&x| x == MAX).unwrap();
  println!("{}", ans);
}