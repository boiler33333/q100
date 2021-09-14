use proconio::input;
use std::usize::MAX;

fn main() {
  input! {
    n: usize, m: usize,
    edge: [(usize, usize, usize, usize); m],
  }
  let mut graph = vec![vec![]; n];
  for (s, t, d, time) in edge {
    graph[s-1].push((t-1, d, time));
    graph[t-1].push((s-1, d, time));
  }
  let mut dp = vec![vec![(MAX, 0usize); 1<<n]; n];
  dp[0][0] = (0, 1);
  for state in 0..1<<n {
    for s in 0..n {
      if dp[s][state].0 == MAX {
        continue;
      }
      for &(t, d, time) in &graph[s] {
        if state & 1 << t > 0 {
          continue;
        }
        if dp[s][state].0 > time - d {
          continue;
        }
        if dp[t][state|1<<t].0 > dp[s][state].0 + d {
          dp[t][state|1<<t].0 = dp[s][state].0 + d;
          dp[t][state|1<<t].1 = dp[s][state].1;
        } else if dp[t][state|1<<t].0 == dp[s][state].0 + d {
          dp[t][state|1<<t].1 += dp[s][state].1;
        } 
      }
    }
  }
  let (ans, cnt) = dp[0][(1<<n)-1];
  if ans == MAX {
    println!("IMPOSSIBLE");
  } else {
    println!("{} {}", ans, cnt);
  }
}