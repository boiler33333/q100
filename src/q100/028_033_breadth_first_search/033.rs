use proconio::input;
use std::collections::VecDeque;

fn bfs(h: usize, w: usize, table: &Vec<Vec<char>>) -> i64 {
  let mut dist = vec![vec![-1; w]; h];
  let mut que: VecDeque<(usize, usize)> = VecDeque::new();
  dist[0][0] = 0;
  que.push_back((0, 0));
  while let Some((x, y)) = que.pop_front() {
    if x == w-1 && y == h-1 {
      break;
    }
    for i in 0..4 {
      if y == 0 && i == 2 || y == h-1 && i == 0 {
        continue;
      }
      if x == 0 && i == 3 || x == w-1 && i == 1 {
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
  dist[h-1][w-1]
}

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [String; h],
  }
  let mut table = vec![vec!['.'; w]; h];
  for y in 0..h {
    for (x, c) in s[y].chars().enumerate() {
      table[y][x] = c;
    }
  }
  let dist = bfs(h, w, &table);
  if dist < 0 {
    println!("{}", dist);
  } else {
    let dist = dist as usize;
    let mut ans = w * h - dist - 1;
    for y in 0..h {
      for x in 0..w {
        if table[y][x] == '#' {
          ans -= 1;
        }
      }
    }
    println!("{}", ans);
  }
}