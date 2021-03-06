use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    c: [usize; n],
  }
  let mut dp = vec![MAX; n+1];
  for i in 0..n {
    let j = dp.binary_search(&c[i]).unwrap_or_else(|x| x);
    dp[j] = c[i];
  }
  let m = dp.iter().position(|&x| x == MAX).unwrap();
  let ans = n - m;
  println!("{}", ans);
}