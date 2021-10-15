use proconio::input;
use std::cmp::Ordering::{ Less, Equal, Greater };
use std::cmp::{max, min};
use std::usize::MAX;

pub trait BinarySearch<T> {
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn upper_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less | Equal => left = mid + 1,
        Greater => right = mid,
      }
    }
    left
  }
}

fn main() {
  input! {
    n: usize,
    k: usize,
    vw: [(usize, usize); n],
  }

  let mut max_w = 0;
  for &(_, w) in &vw {
    max_w = max(max_w, w);
  }

  let ans = if n <= 30 {
    meet_in_the_middle(n, k, &vw)
  } else if max_w <= 1000 {
    dp_w(n, k, &vw)
  } else {
    dp_v(n, k, &vw)
  };
  println!("{}", ans);
}

fn setup(k: usize, vw: &[(usize, usize)]) -> Vec<(usize, usize)> {
  let mut ret = vec![];
  let n = vw.len();
  for state in 0..1<<n {
    let mut v = 0;
    let mut w = 0;
    for i in 0..n {
      if state & 1 << i > 0 {
        v += vw[i].0;
        w += vw[i].1;
      }
    }
    if w <= k {
      ret.push((w, v));
    }
  }
  ret
}

fn meet_in_the_middle(
  n: usize,
  k: usize,
  vw: &[(usize, usize)],
) -> usize {
  let wv1 = setup(k, &vw[0..n/2]);
  let mut wv2 = setup(k, &vw[n/2..n]);
  wv2.sort();

  let mut tmp = 0;
  for (_, v2) in &mut wv2 {
    *v2 = max(*v2, tmp);
    tmp = *v2;
  }
  
  let mut ret = 0;
  for (w1, v1) in wv1 {
    let i = wv2.upper_bound(&(k-w1, MAX));
    let (_, v2) = wv2[i-1];
    ret = max(ret, v1 + v2);
  }
  ret
}

fn dp_w(
  n: usize,
  k: usize,
  vw: &[(usize, usize)],
) -> usize {
  let mut dp = vec![vec![0; n*1000+1]; n+1];
  dp[0][0] = 0;
  for (i, &(v, w)) in vw.iter().enumerate() {
    for j in 0..n*1000+1 {
      if j >= w {
        dp[i+1][j] = max(dp[i+1][j], dp[i][j-w] + v)
      }
      dp[i+1][j] = max(dp[i+1][j], dp[i][j]);
    }
  }
  let mut ret = 0;
  for i in 0..=k {
    ret = max(ret, dp[n][i]);
  }
  ret
}

fn dp_v(
  n: usize,
  k: usize,
  vw: &[(usize, usize)],
) -> usize {
  let mut dp = vec![vec![MAX/2; n*1000+1]; n+1];
  dp[0][0] = 0;
  for (i, &(v, w)) in vw.iter().enumerate() {
    for j in 0..n*1000+1 {
      if j >= v {
        dp[i+1][j] = min(dp[i+1][j], dp[i][j-v] + w);
      }
      dp[i+1][j] = min(dp[i+1][j], dp[i][j]);
    }
  }
  let mut ret = 0;
  for i in 0..n*1000+1 {
    if dp[n][i] <= k {
      ret = max(ret, i);
    }
  }
  ret
}