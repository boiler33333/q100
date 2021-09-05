use proconio::input;
use std::cmp::max;

fn dfs(
  h: usize,
  w: usize,
  g: &mut Vec<Vec<usize>>,
  y: usize,
  x: usize,
  cnt: usize,
  ans: &mut usize,
) {
  g[y][x] = 0;
  *ans = max(*ans, cnt);
  for i in 0..4 {
    if y == 0 && i == 2 || y == h-1 && i == 0 {
      continue;
    }
    if x == 0 && i == 3 || x == w-1 && i == 1 {
      continue;
    }
    let (x2 ,y2) = match i {
      0 => (x + 0, y + 1),
      1 => (x + 1, y + 0),
      2 => (x + 0, y - 1),
      3 => (x - 1, y + 0),
      _ => (x + 0, y + 0),
    };
    if g[y2][x2] == 0 {
      continue;
    }
    dfs(h, w, g, y2, x2, cnt+1, ans);
  }
  g[y][x] = 1;
}

fn main() {
  input! {
    n: usize,
    m: usize,
    mut g: [[usize; m]; n],
  }
  let mut ans = 0;
  for y in 0..n {
    for x in 0..m {
      if g[y][x] == 1 {
        dfs(n, m, &mut g, y, x, 1, &mut ans);
      } 
    }
  }
  println!("{}", ans);
}