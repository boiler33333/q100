use std::cmp::{max, min};
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
    let n: usize = read();
    let w: usize = read();
    let d: usize = read();
    if n == 0 && w == 0 && d == 0 {
      break;
    }
    let mut ps = vec![];
    for _ in 0..n {
      let p: usize = read();
      let s: usize = read();
      ps.push((p, s));
    }
    let ans: Vec<usize> = solve(w, d, &ps);
    let ans: Vec<String> = ans.iter().map(|x| x.to_string()).collect();
    let ans: String = ans.join(" ");
    println!("{}", ans);
  }
}

fn solve(
  w: usize,
  d: usize,
  ps: &[(usize, usize)],
) -> Vec<usize> {
  let mut pieces = vec![(w, d)];
  for &(p, s) in ps {
    let piece = pieces[p-1];
    pieces.remove(p-1);
    let (small, large) = cut(piece, s);
    pieces.push(small);
    pieces.push(large);
  }
  let mut area: Vec<usize> = pieces.iter().map(|(w, d)| w * d).collect();
  area.sort();
  area
}

fn cut((w, d): (usize, usize), s: usize) -> ((usize, usize), (usize, usize)) {
  let s = s % (w + d);
  if s < w {
    let w1 = min(s, w - s);
    let w2 = max(s, w - s);
    ((w1, d), (w2, d))
  } else {
    let d1 = min(s - w, w + d - s);
    let d2 = max(s - w, w + d - s);
    ((w, d1), (w, d2))
  }
}

#[test]
fn test_solve_1() {
  let got = solve(5, 6, &[(1,18),(2,19),(1,2)]);
  let want = vec![4,4,6,16];
  assert_eq!(got, want); 
}

#[test]
fn test_solve_2() {
  let got = solve(4, 1, &[(1,1),(2,1),(3,1)]);
  let want = vec![1,1,1,1];
  assert_eq!(got, want); 
}

#[test]
fn test_solve_3() {
  let got = solve(2, 5, &[]);
  let want = vec![10];
  assert_eq!(got, want); 
}