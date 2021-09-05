use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };
use std::usize::MAX;

trait BinarySearch<T> {
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
    c: [usize; n],
  }
  let mut dp = vec![MAX; n+1];
  for v in c {
    let i = dp.lower_bound(&v);
    dp[i] = v;
  }
  // lis: longest increasing sequence 
  let lis = dp.iter().position(|&x| x == MAX).unwrap();
  let ans = n - lis;
  println!("{}", ans);
}