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

fn main() {
  loop {
    let h: usize = read();
    if h == 0 {
      break;
    }
    let w: usize = 5;
    let k: usize = 3;
    let mut table = vec![vec![0; w]; h];
    for y in 0..h {
      for x in 0..w {
        table[y][x] = read();
      }
    }
    let ans = solve(h, w, k, &mut table);
    println!("{}", ans);
  }
}

fn solve(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> usize {
  let mut points = 0;
  loop {
    let blocks = delete(h, w, k, table);
    if blocks.len() == 0 {
      break;
    }
    for (v, n) in blocks {
      points += v * n;
    }
    fall(h, w, table);
  }
  points
}

fn delete(h: usize, w: usize, k: usize, table: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)> {
  let mut blocks = vec![];
  for y in 0..h {
    for x in 0..w {
      if table[y][x] == 0 {
        continue;
      }
      let mut n = 1;
      while x + n < w && table[y][x] == table[y][x+n] {
        n += 1;
      }
      if n < k {
        continue;
      }
      blocks.push((table[y][x], n));
      for i in 0..n {
        table[y][x+i] = 0;
      }
    }
  }
  blocks
}

fn fall(h: usize, w: usize, table: &mut Vec<Vec<usize>>) {
  for i in 1..h {
    for y in (i..h).rev() {
      for x in 0..w {
        if table[y][x] == 0 {
          let tmp = table[y][x];
          table[y][x] = table[y-1][x];
          table[y-1][x] = tmp;
        }
      }
    }
  }
}

#[test]
fn test_solve_1() {
  let mut table = vec![
    vec![6,9,9,9,9],
  ];
  let got = solve(1, 5, 3, &mut table);
  assert_eq!(got, 36);
}

#[test]
fn test_solve_2() {
  let mut table = vec![
    vec![5,9,5,5,9],
    vec![5,5,6,9,9],
    vec![4,6,3,6,9],
    vec![3,3,2,9,9],
    vec![2,2,1,1,1],
  ];
  let got = solve(5, 5, 3, &mut table);
  assert_eq!(got, 38);
}

#[test]
fn test_solve_3() {
  let mut table = vec![
    vec![3,5,6,5,6],
    vec![2,2,2,8,3],
    vec![6,2,5,9,2],
    vec![7,7,7,6,1],
    vec![4,6,6,4,9],
    vec![8,9,1,1,8],
    vec![5,6,1,8,1],
    vec![6,8,2,1,2],
    vec![9,6,3,3,5],
    vec![5,3,8,8,8],
  ];
  let got = solve(10, 5, 3, &mut table);
  assert_eq!(got, 99);
}

#[test]
fn test_solve_4() {
  let mut table = vec![
    vec![1,2,3,4,5],
    vec![6,7,8,9,1],
    vec![2,3,4,5,6],
    vec![7,8,9,1,2],
    vec![3,4,5,6,7],
  ];
  let got = solve(5, 5, 3, &mut table);
  assert_eq!(got, 0);
}

#[test]
fn test_solve_5() {
  let mut table = vec![
    vec![2,2,8,7,4],
    vec![6,5,7,7,7],
    vec![8,8,9,9,9],
  ];
  let got = solve(3, 5, 3, &mut table);
  assert_eq!(got, 72);
}