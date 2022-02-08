use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<usize>>,
) -> usize {
  let mut cnt = 0;
  let mut dist = vec![vec![MAX; w]; h];
  let mut que = VecDeque::new();
  dist[0][0] = 0;
  que.push_back((0, 0));
  while let Some((ux, uy)) = que.pop_front() {
    if uy % 2 == 0 {
      for i in 0..6 {
        if uy == 0 && i <= 1 || uy == h-1 && i >= 4 {
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
          5 => (ux + 0, uy + 1),
          _ => unreachable!(),
        };
        if table[vy][vx] > 0 {
          cnt += 1;
          continue;
        }
        if dist[vy][vx] == MAX {
          dist[vy][vx] = dist[uy][ux] + 1;
          que.push_back((vx, vy));
        }
      }
    } else {
      for i in 0..6 {
        if uy == 0 && i <= 1 || uy == h-1 && i >= 4 {
          continue;
        }
        if ux == 0 && i == 2 || ux == w-1 && i % 2 == 1 {
          continue;
        }
        let (vx, vy) = match i {
          0 => (ux + 0, uy - 1),
          1 => (ux + 1, uy - 1),
          2 => (ux - 1, uy + 0),
          3 => (ux + 1, uy + 0),
          4 => (ux + 0, uy + 1),
          5 => (ux + 1, uy + 1),
          _ => unreachable!(),
        };
        if table[vy][vx] > 0 {
          cnt += 1;
          continue;
        }
        if dist[vy][vx] == MAX {
          dist[vy][vx] = dist[uy][ux] + 1;
          que.push_back((vx, vy));
        }
      }
    }
  }
  cnt
}

fn main() {
  input! {
    w: usize, h: usize,
    a: [[usize; w]; h],
  }
  let mut table = vec![vec![0; w+2]; h+2];
  for y in 0..h {
    for x in 0..w {
      table[y+1][x+1] = a[y][x];
    }
  }
  let ans = bfs(h+2, w+2, &mut table);
  println!("{}", ans);
}