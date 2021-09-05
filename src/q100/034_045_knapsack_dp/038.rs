use std::cmp::max;
use std::io::*;
use std::str::FromStr;

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

fn main() {
  let q = read::<usize>();
  for _ in 0..q {
    let x: Vec<char> = read::<String>().chars().collect();
    let y: Vec<char> = read::<String>().chars().collect();
    let n = y.len();
    let m = x.len();
    let mut dp = vec![vec![0; m+1]; n+1];
    for i in 0..n {
      for j in 0..m {
        if y[i] == x[j] {
          dp[i+1][j+1] = dp[i][j] + 1;
        } else {
          dp[i+1][j+1] = max(dp[i+1][j], dp[i][j+1]);
        }
      }
    }
    println!("{}", dp[n][m]);
  }
}