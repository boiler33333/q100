use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    n: usize, m: usize,
    xy: [(usize, usize); m],
  }
  let mut table = vec![vec![false; n]; n];
  for &(x, y) in &xy {
    table[x-1][y-1] = true;
    table[y-1][x-1] = true;
  }
  let mut ans = 0;
  for status in 0..1<<n {
    let mut ok = true;
    let mut cnt = 0;
    for y in 0..n {
      if status & 1 << y == 0 {
        continue;
      }
      for x in 0..n {
        if x == y {
          continue;
        }
        if status & 1 << x == 0 {
          continue;
        } 
        if table[y][x] {
          continue;
        }
        ok = false;
      }
      cnt += 1;
    }
    if ok {
      ans = max(ans, cnt);
    }
  }
  println!("{}", ans);
}