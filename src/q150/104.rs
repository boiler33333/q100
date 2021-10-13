use proconio::input;
use std::collections::VecDeque;

fn main() {
  input! {
    w: usize,
    h: usize,
    n: usize,
    a: [(usize, usize, usize, usize); n],
  }
  let ans = solve(h, w, n, &a);
  println!("{}", ans);
}

fn solve(
  h: usize,
  w: usize,
  _: usize,
  a: &[(usize, usize, usize, usize)],
) -> usize {
  let mut x = vec![0, w];
  let mut y = vec![0, h];
  for &(x1, y1, x2, y2) in a {
    x.push(x1); x.push(x2);
    y.push(y1); y.push(y2);
  }
  x.sort(); x.dedup();
  y.sort(); y.dedup();
  let w = x.len();
  let h = y.len();
  let mut table = vec![vec![false; w]; h];
  for &(x1, y1, x2, y2) in a {
    let x1 = x.binary_search(&x1).unwrap();
    let y1 = y.binary_search(&y1).unwrap();
    let x2 = x.binary_search(&x2).unwrap();
    let y2 = y.binary_search(&y2).unwrap();
    for i in y1..y2 {
      for j in x1..x2 {
        table[i][j] = true;
      }
    }
  }
  let mut ret = 0;
  for i in 0..h-1 {
    for j in 0..w-1 {
      if !table[i][j] {
        ret += 1;
        bfs(h-1, w-1, &mut table, (j, i));
      }
    }
  }
  ret
}

fn bfs(
  h: usize,
  w: usize,
  table: &mut Vec<Vec<bool>>,
  (sx, sy): (usize, usize),
){
  let mut que = VecDeque::new();
  que.push_back((sx, sy));
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
        _ => (ux - 1, uy + 0),
      };
      if !table[vy][vx] {
        table[vy][vx] = true;
        que.push_back((vx, vy));
      }
    }
  }
}

#[test]
fn test_solve_1() {
  let w = 15;
  let h = 6;
  let n = 10;
  let a = vec![
    (1, 4, 5, 6),
    (2, 1, 4, 5),
    (1, 0, 5, 1),
    (6, 1, 7, 5),
    (7, 5, 9, 6),
    (7, 0, 9, 2),
    (9, 1, 10, 6),
    (11, 0, 14, 1),
    (12, 1, 13, 5),
    (11, 5, 14, 6),
  ];
  let got = solve(h, w, n, &a);
  assert_eq!(got, 5);
}