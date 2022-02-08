use proconio::input;
use std::cmp::min;
use std::usize::MAX;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; n],
  }
  let mut acc = vec![vec![0usize; n+1]; m];
  for i in 0..n {
    acc[a[i]-1][i+1] = 1;
  }
  for j in 0..m {
    for i in 0..n {
      acc[j][i+1] += acc[j][i];
    }
  }
  let mut dp = vec![MAX; 1<<m];
  dp[0] = 0;
  for state in 0..1<<m {
    if dp[state] == MAX {
      continue;
    }
    let mut i = 0; //並べ直し終わった種類のぬいぐるみの合計
    for j in 0..m {
      if state & 1<<j > 0 {
        i += acc[j][n];
      }
    }
    for j in 0..m {
      if state & 1<<j > 0 {
        continue;
      }
      let cnt = acc[j][n];
      let keep_cnt = acc[j][i + cnt] - acc[j][i];
      let swap_cnt = cnt - keep_cnt;
      dp[state|1<<j] = min(dp[state|1<<j], dp[state] + swap_cnt);
    }
  }
  println!("{}", dp[(1<<m)-1]);
}