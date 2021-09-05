use proconio::input;
use std::collections::VecDeque;

fn bfs(
  r: usize,
  c: usize,
  table: &Vec<Vec<char>>,
  sy: usize,
  sx: usize,
  gy: usize,
  gx: usize,
) -> usize {
  let mut dist: Vec<Vec<i64>> = vec![vec![-1; c]; r];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  dist[sy][sx] = 0;
  que.push_back((sx, sy));
  while let Some((x, y)) = que.pop_front() {
    if x == gx && y == gy {
      break;
    }
    for i in 0..4 {
      if y == 0 && i == 2 || y == r - 1 && i == 0{
        continue;
      }
      if x == 0 && i == 3 || x == c - 1 && i == 1 {
        continue;
      }
      let (x2, y2) = match i {
        0 => (x + 0, y + 1),
        1 => (x + 1, y + 0),
        2 => (x + 0, y - 1),
        _ => (x - 1, y + 0),
      };
      if table[y2][x2] == '#' || dist[y2][x2] >= 0 {
        continue;
      }
      dist[y2][x2] = dist[y][x] + 1;
      que.push_back((x2, y2));
    }
  }
  dist[gy][gx] as usize
}

fn main() {
  input! {
    r: usize,
    c: usize,
    sy: usize,
    sx: usize,
    gy: usize,
    gx: usize,
    s: [String; r],
  }
  let mut table: Vec<Vec<char>> = vec![vec!['.'; c]; r];
  for y in 0..r {
    for (x, c) in s[y].chars().enumerate() {
      table[y][x] = c;
    }
  }
  let ans = bfs(r, c, &table, sy-1, sx-1, gy-1, gx-1);
  println!("{}", ans);
}