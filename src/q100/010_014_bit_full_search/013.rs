use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    r: usize, c: usize,
    a: [[usize; c]; r],
  }
  let mut ans = 0;
  for status in 0..1<<r {
    let mut table = a.clone();
    for y in 0..r {
      if status & 1 << y != 0 {
        for x in 0..c {
          table[y][x] = 1 - table[y][x];
        }
      }
    }
    let mut cnt = 0;
    for x in 0..c {
      let mut heads = 0;
      for y in 0..r {
        if table[y][x] == 0 {
          heads += 1;
        }
      }
      let tails = r - heads;
      cnt += max(heads, tails);
    }
    ans = max(ans ,cnt);
  }
  println!("{}", ans);
}