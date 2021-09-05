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
  let n: usize = read();
  let mut s: Vec<usize> = vec![0; n];
  for i in 0..n {
    s[i] = read();
  }
  let q: usize = read();
  let mut t: Vec<usize> = vec![0; q];
  for j in 0..q {
    t[j] = read();
  }
  let ans = solve(&s, &t);
  println!("{}", ans);
}

fn solve(s: &[usize], t: &[usize]) -> usize {
  let mut cnt = 0;
  for v in t {
    match s.binary_search(v) {
      Ok(_) => cnt += 1,
      Err(_)=> {},
    }
  }
  cnt
}

#[test]
fn test_solve_1() {
  let s = vec![1, 2, 3, 4, 5];
  let t = vec![3, 4, 1];
  let result = solve(&s, &t);
  assert_eq!(result, 3);
}

#[test]
fn test_solve_2() {
  let s = vec![1, 2, 3];
  let t = vec![5];
  let result = solve(&s, &t);
  assert_eq!(result, 0);
}

#[test]
fn test_solve_3() {
  let s = vec![1, 1, 2, 2, 3];
  let t = vec![1, 2];
  let result = solve(&s, &t);
  assert_eq!(result, 2);
}