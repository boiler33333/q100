use std::cmp::Ordering::{ Less, Equal, Greater };
use std::io::*;
use std::str::FromStr;
use std::usize::MAX;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

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
  let n = read::<usize>();
  let mut dp = vec![MAX; n+1];
  for _ in 0..n {
    let a = read::<usize>();
    let i = dp.lower_bound(&a);
    dp[i] = a;
  }
  let ans = dp.iter().position(|&x| x == MAX).unwrap();
  println!("{}", ans);
}