use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize,
    m: usize,
    t: [i64; n],
  }
  let mut a = vec![0; m];
  let mut b = vec![0; m];
  let mut c = vec![0; m];
  for j in 0..m {
    input! { aj: i64, bj: i64, cj: i64 }
    a[j] = aj;
    b[j] = bj;
    c[j] = cj;
  }
  let mut suit = vec![vec![false; m]; n];
  for i in 0..n {
    for j in 0..m {
      suit[i][j] = a[j] <= t[i] && t[i] <= b[j];
    }
  }
  let mut dp = vec![vec![0; m]; n];
  for i in 0..n-1 {
    for j in 0..m {
      if !suit[i][j] {
        continue;
      }
      for k in 0..m {
        if suit[i+1][k] {
          dp[i+1][k] = max(dp[i+1][k], dp[i][j] + (c[j] - c[k]).abs());
        }
      }
    }
  }
  let ans = dp[n-1].iter().max().unwrap();
  println!("{}", ans);
}