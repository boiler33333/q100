use proconio::input;
use std::collections::VecDeque;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<char>>,
  sy: usize,
  sx: usize,
  gy: usize,
  gx: usize,
) -> usize {
  let mut dist: Vec<Vec<i64>> = vec![vec![-1; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  dist[sy][sx] = 0;
  que.push_back((sx, sy));
  while let Some((x, y)) = que.pop_front() {
    if x == gx && y == gy {
      break;
    }
    for i in 0..4 {
      if y == 0 && i == 2 || y == h - 1 && i == 0{
        continue;
      }
      if x == 0 && i == 3 || x == w - 1 && i == 1 {
        continue;
      }
      let (x2, y2) = match i {
        0 => (x + 0, y + 1),
        1 => (x + 1, y + 0),
        2 => (x + 0, y - 1),
        _ => (x - 1, y + 0),
      };
      if table[y2][x2] == 'X' || dist[y2][x2] >= 0 {
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
    h: usize,
    w: usize,
    n: usize,
    s: [String; h],
  }
  let mut table: Vec<Vec<char>> = vec![vec!['.'; w]; h];
  for y in 0..h {
    for (x, c) in s[y].chars().enumerate() {
      table[y][x] = c;
    }
  }
  let mut p = vec![(0, 0); n+1];
  for y in 0..h {
    for x in 0..w {
      match table[y][x] {
        'X' | '.' => {
        },
        'S' => {
          p[0] = (x, y);
        },
        c => {
          let i = c.to_digit(10).unwrap() as usize;
          p[i] = (x, y);
        }
      }
    }
  }
  let mut ans = 0;
  for i in 0..n {
    ans += bfs(h, w, &table, p[i].1, p[i].0, p[i+1].1, p[i+1].0);
  }
  println!("{}", ans);
}