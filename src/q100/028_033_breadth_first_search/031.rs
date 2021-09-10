use proconio::input;
use std::collections::VecDeque;

fn bfs(
  h: usize,
  w: usize,
  building: &Vec<Vec<usize>>,
  sy: usize,
  sx: usize,
) -> usize {
  let mut cnt = 0;
  let mut seen = vec![vec![false; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  seen[sy][sx] = true;
  que.push_back((sx, sy));
  while let Some((ux, uy)) = que.pop_front() {
    if uy % 2 == 0 {
      for i in 0..6 {
        if uy == 0 && i <= 1 || uy == h-1 && 4 <= i {
          continue;
        }
        if ux == 0 && i % 2 == 0 || ux == w-1 && i == 3 {
          continue;
        }
        let (vx, vy) = match i {
          0 => (ux - 1, uy - 1),
          1 => (ux + 0, uy - 1),
          2 => (ux - 1, uy + 0),
          3 => (ux + 1, uy + 0),
          4 => (ux - 1, uy + 1),
          _ => (ux + 0, uy + 1),
        };
        if building[vy][vx] == 1 {
          cnt += 1;
          continue;
        }
        if !seen[vy][vx] {
          seen[vy][vx] = true;
          que.push_back((vx, vy));
        }
      }
    } else {
      for i in 0..6 {
        if uy == 0 && i <= 1 || uy == h-1 && 4 <= i {
          continue;
        }
        if ux == 0 && i == 2 || ux == w-1 && i % 2 == 1 {
          continue;
        }
        let (vx, vy) = match i {
          0 => (ux - 0, uy - 1),
          1 => (ux + 1, uy - 1),
          2 => (ux - 1, uy + 0),
          3 => (ux + 1, uy + 0),
          4 => (ux - 0, uy + 1),
          _ => (ux + 1, uy + 1),
        };
        if building[vy][vx] == 1 {
          cnt += 1;
          continue;
        }
        if !seen[vy][vx] {
          seen[vy][vx] = true;
          que.push_back((vx, vy));
        }
      }
    }
  }
  cnt
}

fn main() {
  input! {
    mut w: usize,
    mut h: usize,
    mut table: [[usize; w]; h],
  }
  table.insert(0, vec![0; w]);
  table.push(vec![0; w]);
  h = table.len();
  for y in 0..h {
    table[y].insert(0, 0);
    table[y].push(0);
  }
  w = table[0].len();
  let ans = bfs(h, w, &table, 0, 0);
  println!("{}", ans);
}