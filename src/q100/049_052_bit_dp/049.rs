use std::cmp::min;
use std::usize::MAX;
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
  let v: usize = read();
  let e: usize = read();
  let mut g = vec![vec![]; v];
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let d: usize = read();
    g[s].push((t, d));
  }
  let mut dp = vec![vec![MAX; v]; 1<<v];
  dp[0][0] = 0;
  for state in 0..1<<v {
    for from in 0..v {
      if dp[state][from] == MAX {
        continue;
      }
      for &(to, d) in &g[from] {
        if state & (1 << to) > 0 {
          continue;
        }
        dp[state|1<<to][to] = min(dp[state|1<<to][to], dp[state][from] + d);
      }
    }
  }
  let ans = dp[(1<<v)-1][0];
  if ans == MAX {
    println!("-1");
  } else {
    println!("{}", ans);
  }
}