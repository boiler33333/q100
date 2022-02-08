use proconio::input;
use std::cmp::max;

fn cumulative_sum(h: usize, w: usize, table: &mut Vec<Vec<usize>>) {
  for y in 0..h {
    for x in 1..w {
      table[y][x] += table[y][x-1];
    }
  }
  for y in 1..h {
    for x in 0..w {
      table[y][x] += table[y-1][x];
    }
  }
}

fn calc(a: usize, b: usize, c: usize, d: usize, table: &Vec<Vec<usize>>) -> usize {
  let a = a - 1;
  let b = b - 1;
  table[c][d] + table[a][b] - table[a][d] - table[c][b]
}

fn main() {
  input! {
    h: usize,
    w: usize,
    k: usize,
    v: usize,
    a: [[usize; w]; h],
  }
  let mut table = vec![vec![0; w+1]; h+1];
  for y in 0..h {
    for x in 0..w {
      table[y+1][x+1] = a[y][x];
    }
  }
  cumulative_sum(h+1, w+1, &mut table);
  let mut ans = 0;
  for y2 in 1..=h {
    for y1 in 1..=y2 {
      for x2 in 1..=w {
        for x1 in 1..=x2 {
          let s = (y2 - y1 + 1) * (x2 - x1 + 1);
          let a = calc(y1, x1, y2, x2, &table);
          if a + s * k <= v {
            ans = max(ans, s);
          }
        }
      }
    }
  }
  println!("{}", ans);
}