use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };

pub trait BinarySearch<T> {
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
    x: [usize; n],
    k: usize,
    q: usize,
    ab: [(usize, usize); q],
  }
  let mut dp = vec![vec![0; n]; 30];
  for i in 0..n {
    dp[0][i] = x.upper_bound(&(x[i]+k)) - 1;
  }
  for i in 0..29 {
    for j in 0..n {
      dp[i+1][j] = dp[i][dp[i][j]];
    }
  }
  for &(a, b) in &ab {
    let (a, b) = if a < b { (a-1, b-1) } else { (b-1, a-1) };
    let mut pos = a;
    let mut ans = 0;
    for i in (0..30).rev() {
      if dp[i][pos] < b {
        pos = dp[i][pos];
        ans += 1 << i;
      }
    }
    println!("{}", ans+1);
  }
}