use std::cmp::max;
use std::cmp::min;
use std::io::*;
use std::str::FromStr;
use std::i64::MAX;

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
  loop {
    let n: usize = read();
    let m: usize = read();
    if n == 0 && m == 0 {
      break;
    }
    let mut c: Vec<i64> = vec![0; m];
    let mut x: Vec<i64> = vec![0; n];
    for j in 0..m {
      c[j] = read();
    }
    for i in 0..n {
      x[i] = read();
    }
    solve(n, m, &c, &x);
  }
}

fn solve(n: usize, m: usize, c: &Vec<i64>, x: &Vec<i64>) {
  let mut dp: Vec<Vec<i64>> = vec![vec![MAX; 256]; n+1];
  dp[0][128] = 0;
  for i in 0..n {
    for y in 0..256 {
      if dp[i][y] == MAX {
        continue;
      }
      for j in 0..m {
        let y1 = (y as i64) + c[j];
        let y1 = min(max(0, y1), 255);
        let d = y1 - x[i];
        let y1 = y1 as usize;
        dp[i+1][y1] = min(dp[i+1][y1], dp[i][y] + d * d);
      }
    }
  }
  let ans = dp[n].iter().min().unwrap();
  println!("{}", ans);
}