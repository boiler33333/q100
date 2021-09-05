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
  let mut a: Vec<usize> = vec![0; n];
  for i in 0..n {
    a[i] = read();
  }
  let q: usize = read();
  let mut m: Vec<usize> = vec![0; q];
  for j in 0..q {
    m[j] = read();
  }
  let result = solve(n, &a, q, &m);
  for ok in result {
    println!("{}", ok);
  }
}

fn solve(n: usize, a: &[usize], q: usize, m: &[usize]) -> Vec<String> {
  let mut res = vec![String::from("no"); q];
  for status in 0..1<<n {
    let mut sum = 0;
    for i in 0..n {
      if status & 1 << i != 0 {
        sum += a[i];
      }
    }
    for j in 0..q {
      if m[j] == sum {
        res[j] = String::from("yes");
      }
    }
  }
  res
}

#[test]
fn test_solve_1() {
  let n = 5;
  let s = vec![1,5,7,10,21];
  let q = 4;
  let t = vec![2,4,17,8];
  let got = solve(n, &s, q, &t);
  let want = vec!["no", "no", "yes", "yes"];
  for j in 0..q {
    assert_eq!(got[j], want[j]);
  }
}