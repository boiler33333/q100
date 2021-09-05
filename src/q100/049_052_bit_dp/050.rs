use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    m: usize,
    e: [(usize, usize, usize, usize); m],
  }
  let mut g = vec![vec![]; n];
  for &(s, t, d, time) in &e {
    g[s-1].push((t-1, d, time));
    g[t-1].push((s-1, d, time));
  }
  let mut dp: Vec<Vec<(usize, usize)>> = vec![vec![(MAX, 0); n]; 1 << n];
  dp[0][0] = (0, 1);
  for state in 0..(1 << n) {
    for from in 0..n {
      if dp[state][from].0 == MAX {
        continue;
      }
      for &(to, d, time) in &g[from] {
        if state & (1 << to) > 0 {
          continue;
        }
        if dp[state][from].0 + d > time {
          continue;
        }
        if dp[state|1<<to][to].0 > dp[state][from].0 + d {
          dp[state|1<<to][to].0 = dp[state][from].0 + d;
          dp[state|1<<to][to].1 = dp[state][from].1;
        } else if dp[state|1<<to][to].0 == dp[state][from].0 + d {
          dp[state|1<<to][to].1 += dp[state][from].1;
        }
      }
    }
  }
  let (ans, cnt) = dp[(1<<n)-1][0];
  if ans == MAX {
    println!("IMPOSSIBLE");
  } else {
    println!("{} {}", ans, cnt);
  }
}