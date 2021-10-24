use std::cmp::min;
use std::collections::HashSet;
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
  let mut dp = vec![vec![0; 201]; 201];
  for i in 0..201 {
    for w in (0..=i).rev() {
      let b = i - w;
      let mut hs = HashSet::new();
      if w > 0 {
        hs.insert(dp[w-1][b]);
      }
      for j in 1..=min(w, b) {
        hs.insert(dp[w][b-j]);
      }
      if b > 0 {
        hs.insert(dp[w+1][b-1]);
      }
      while hs.contains(&dp[w][b]) {
        dp[w][b] += 1;
      }
    }
  }
  let n: usize = read();
  let mut grundy = 0;
  for _ in 0..n {
    let w: usize = read();
    let b: usize = read();
    grundy ^= dp[w][b];
  }
  let ans = if grundy > 0 { 0 } else { 1 };
  println!("{}", ans);
}