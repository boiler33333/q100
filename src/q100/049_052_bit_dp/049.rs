use std::cmp::min;
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

fn main() {
  let v: usize = read();
  let e: usize = read();
  let mut graph = vec![vec![]; v];
  for _ in 0..e {
    let s: usize = read();
    let t: usize = read();
    let d: usize = read();
    graph[s].push((t, d));
  }
  let mut dp = vec![vec![MAX; 1<<v]; v];
  dp[0][0] = 0;
  for state in 0..1<<v {
    for s in 0..v {
      if dp[s][state] == MAX {
        continue;
      }
      for &(t, d) in &graph[s] {
        if state & 1 << t > 0 {
          continue;
        }
        dp[t][state|1<<t] = min(dp[t][state|1<<t], dp[s][state] + d);
      }
    }
  }
  let ans = dp[0][(1<<v)-1];
  if ans == MAX {
    println!("-1");
  } else {
    println!("{}", ans);
  }
}