use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    r: usize, c: usize,
    a: [[usize; c]; r],
  }
  let mut ans = 0;
  for state in 0..1<<r {
    let mut table = a.clone();
    for y in 0..r {
      if state & 1 << y > 0 {
        for x in 0..c {
          table[y][x] = 1 - table[y][x];
        }
      }
    }
    let mut sum = 0;
    for x in 0..c {
      let mut cnt = 0;
      for y in 0..r {
        if table[y][x] > 0 {
          cnt += 1;
        }
      }
      sum += max(cnt, r - cnt);
    }
    ans = max(ans, sum);
  }
  println!("{}", ans);
}