use proconio::input;
use std::collections::VecDeque;
use std::usize::MAX;

fn bfs(
  h: usize,
  w: usize,
  table: &Vec<Vec<char>>,
) -> usize {
  let mut dist = vec![vec![MAX; w]; h];
  let mut que = VecDeque::new();
  dist[0][0] = 0;
  que.push_back((0, 0));
  while let Some((ux, uy)) = que.pop_front() {
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
      if table[vy][vx] == '#' {
        continue;
      }
      if dist[vy][vx] == MAX {
        dist[vy][vx] = dist[uy][ux] + 1;
        que.push_back((vx, vy));
      }
    }
  }
  dist[h-1][w-1]
}

fn main() {
  input! {
    h: usize, w: usize,
    s: [String; h],
  }
  let s = s.iter().map(|x| x.chars().collect()).collect();
  let dist = bfs(h, w, &s);
  if dist == MAX {
    println!("-1");
  } else {
    let mut ans = h * w - dist - 1;
    for y in 0..h {
      for x in 0..w {
        if s[y][x] == '#' {
          ans -= 1;
        }
      }
    }
    println!("{}", ans);
  }
}