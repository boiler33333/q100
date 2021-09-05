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

fn solve(n: usize, w: &Vec<i64>) {
  let mut dp = vec![vec![0; n]; n];
  for d in 2..=n {
    if d % 2 == 1 {
      continue;
    }
    for l in 0..=n-d {
      let r = l + d - 1; 
      if dp[l+1][r-1] == d - 2 {
        if (w[l] - w[r]).abs() <= 1 {
          dp[l][r] = d;
        } else {
          dp[l][r] = d - 2;
        }
      }
      for k in l+1..r-1 {
        dp[l][r] = max(dp[l][r], dp[l][k] + dp[k+1][r]);
      }
    }
  }
  if n % 2 == 1 {
    println!("{}", max(dp[0][n-2], dp[1][n-1]));
  } else {
    println!("{}", dp[0][n-1]);
  }
}

fn main() {
  loop {
    let n: usize = read();
    if n == 0 {
      break;
    }
    let mut w: Vec<i64> = vec![0; n];
    for i in 0..n {
      w[i] = read();
    }
    solve(n, &w);
  }
}