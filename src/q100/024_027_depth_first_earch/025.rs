use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
  let s = stdin();
  let s = s.lock();
  let s: String = s.bytes()
    .map(|c| c.expect("failed reading char") as char)
    .skip_while(|c| c.is_whitespace())
    .take_while(|c| !c.is_whitespace())
    .collect();
  s.parse().ok().expect("failed parsing")
}

fn dfs(
  h: usize,
  w: usize,
  table: &mut Vec<Vec<usize>>,
  (x1, y1): (usize, usize),
){
  for i in 0..9 {
    if y1 == 0 && i <= 2 || y1 == h-1 && i >= 6 {
      continue;
    }
    if x1 == 0 && i % 3 == 0 || x1 == w-1 && i % 3 == 2 {
      continue;
    }
    let (x2, y2) = match i {
      0 => (x1 - 1, y1 - 1),
      1 => (x1 + 0, y1 - 1),
      2 => (x1 + 1, y1 - 1),
      3 => (x1 - 1, y1 + 0),
      4 => (x1 + 0, y1 + 0),
      5 => (x1 + 1, y1 + 0),
      6 => (x1 - 1, y1 + 1),
      7 => (x1 + 0, y1 + 1),
      8 => (x1 + 1, y1 + 1),
      _ => unreachable!(),
    };
    if table[y2][x2] > 0 {
      table[y2][x2] = 0;
      dfs(h, w, table, (x2, y2));
    }
  }
}

fn main() {
  loop {
    let w: usize = read();
    let h: usize = read();
    if w == 0 && h == 0 {
      break;
    }
    let mut c = vec![vec![0; w]; h];
    for y in 0..h {
      for x in 0..w {
        c[y][x] = read();
      }
    }
    let ans = solve(h, w, &mut c);
    println!("{}", ans);
  }
}

fn solve(h: usize, w: usize, table: &mut Vec<Vec<usize>>) -> usize {
  let mut cnt = 0;
  for y in 0..h {
    for x in 0..w {
      if table[y][x] > 0 {
        cnt += 1;
        dfs(h, w, table, (x, y))
      }
    }
  }
  cnt
}

#[test]
fn test_solve_0() {
  let w = 1;
  let h = 1;
  let mut c = vec![vec![0]];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 0);
}

#[test]
fn test_solve_1() {
  let w = 2;
  let h = 2;
  let mut c = vec![
    vec![0,1],
    vec![1,0],
  ];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 1);
}

#[test]
fn test_solve_2() {
  let w = 3;
  let h = 2;
  let mut c = vec![
    vec![1,1,1],
    vec![1,1,1]
  ];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 1);
}

#[test]
fn test_solve_3() {
  let w = 5;
  let h = 4;
  let mut c = vec![
    vec![1,0,1,0,0],
    vec![1,0,0,0,0],
    vec![1,0,1,0,1],
    vec![1,0,0,1,0],
  ];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 3);
}

#[test]
fn test_solve_4() {
  let w = 5;
  let h = 4;
  let mut c = vec![
    vec![1,1,1,0,1],
    vec![1,0,1,0,1],
    vec![1,0,1,0,1],
    vec![1,0,1,1,1],
  ];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 1);
}

#[test]
fn test_solve_5() {
  let w = 5;
  let h = 5;
  let mut c = vec![
    vec![1,0,1,0,1],
    vec![0,0,0,0,0],
    vec![1,0,1,0,1],
    vec![0,0,0,0,0],
    vec![1,0,1,0,1],
  ];
  let result = solve(h, w, &mut c);
  assert_eq!(result, 9);
}