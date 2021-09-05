use proconio::input;
use std::collections::VecDeque;

fn bfs(h: usize, w: usize, g: &Vec<Vec<usize>>) -> usize {
  let mut cnt = 0;
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  let mut seen = vec![vec![false; w]; h];
  que.push_back((0, 0));
  seen[0][0] = true;
  while let Some((x, y)) = que.pop_front() {
    if y % 2 == 0 {
      for i in 0..6 {
        if y == 0 && i <= 1 || y == h-1 && i >= 4 {
          continue;
        }
        if x == 0 && i % 2 == 0 || x == w-1 && i == 3 {
          continue;
        }
        let (x2, y2) = match i {
          0 => (x - 1, y - 1),
          1 => (x + 0, y - 1),
          2 => (x - 1, y + 0),
          3 => (x + 1, y + 0),
          4 => (x - 1, y + 1),
          _ => (x + 0, y + 1),
        };
        if seen[y2][x2] {
          continue;
        }
        if g[y2][x2] > 0 {
          cnt += 1;
          continue;
        }
        seen[y2][x2] = true;
        que.push_back((x2, y2));
      }
    } else {
      for i in 0..6 {
        if y == 0 && i <= 1 || y == h-1 && i >= 4 {
          continue;
        }
        if x == 0 && i == 2 || x == w-1 && i % 2 == 1 {
          continue;
        }
        let (x2, y2) = match i {
          0 => (x + 0, y - 1),
          1 => (x + 1, y - 1),
          2 => (x - 1, y + 0),
          3 => (x + 1, y + 0),
          4 => (x + 0, y + 1),
          _ => (x + 1, y + 1),
        };
        if seen[y2][x2] {
          continue;
        }
        if g[y2][x2] > 0 {
          cnt += 1;
          continue;
        }
        seen[y2][x2] = true;
        que.push_back((x2, y2));
      }
    }
  }
  cnt
}

fn main() {
  input! {
    w: usize,
    h: usize,
    mut g: [[usize; w]; h],
  }
  for i in 0..h {
    g[i].insert(0, 0);
    g[i].push(0);
  }
  g.insert(0, vec![0; w+2]);
  g.push(vec![0; w+2]);
  let ans = bfs(h+2, w+2, &g);
  println!("{}", ans);
}