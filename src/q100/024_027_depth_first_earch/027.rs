use proconio::input;
use std::cmp::max;

fn dfs(
  w: usize,
  h: usize,
  table: &mut Vec<Vec<usize>>,
  (ux, uy): (usize, usize),
  ans: &mut usize,
  cnt: usize,
) {
  table[uy][ux] = 0;
  *ans = max(*ans, cnt);
  for i in 0..4 {
    if uy == 0 && i == 0 || uy == h-1 && i == 2 {
      continue;
    }
    if ux == 0 && i == 3 || ux == w-1 && i == 1 {
      continue;
    }
    let (vx, vy) = match i {
      0 => (ux + 0, uy - 1),
      1 => (ux + 1, uy + 0),
      2 => (ux + 0, uy + 1),
      3 => (ux - 1, uy + 0),
      _ => unreachable!(),
    };
    if table[vy][vx] > 0 {
      dfs(w, h, table, (vx, vy), ans, cnt+1);
    }
  }
  table[uy][ux] = 1;
}

fn main() {
  input! {
    w: usize,
    h: usize,
    mut table: [[usize; w]; h],
  }
  let mut ans = 0;
  for y in 0..h {
    for x in 0..w {
      if table[y][x] > 0 {
        dfs(w, h, &mut table, (x,y), &mut ans, 1);
      }
    }
  }
  println!("{}", ans);
}