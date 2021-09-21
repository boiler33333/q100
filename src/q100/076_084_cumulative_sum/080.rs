use proconio::input;
use std::cmp::max;

fn main() {
  input! {
    h: usize,
    w: usize,
    k: usize,
    v: usize,
    mut a: [[usize; w]; h],
  }
  let ans = solve(h, w, k, v, &mut a);
  println!("{}", ans);
}

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

fn calc((a, b, c, d): (usize, usize, usize, usize), table: &Vec<Vec<usize>>) -> usize {
  table[c][d] + table[a][b] - table[c][b] - table[a][d]
}

fn solve(
  h: usize,
  w: usize,
  k: usize,
  v: usize,
  a: &mut Vec<Vec<usize>>,
) -> usize {
  let h = h + 1;
  let w = w + 1;
  for y in 0..h-1 {
    a[y].insert(0, 0);
  }
  a.insert(0, vec![0; w]);
  cumulative_sum(h, w, a);
  let mut ret = 0;
  for y2 in 1..h {
    for y1 in 0..y2 {
      for x2 in 1..w {
        for x1 in 0..x2 {
          let s = (y2-y1) * (x2-x1);
          let b = calc((y1, x1, y2, x2), &a);
          let cost = b + s * k; 
          if cost <= v {
            ret = max(ret, s);
          }
        }
      }
    }
  }
  ret
}