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

fn mul(
  a: &Vec<Vec<usize>>,
  b: &Vec<Vec<usize>>,
  m: usize,
) -> Vec<Vec<usize>> {
  let mut ret = vec![
    vec![0, 0],
    vec![0, 0],
  ];
  for i in 0..2 {
    for j in 0..2 {
      for k in 0..2 {
        ret[i][j] += a[i][k] * b[k][j];
        ret[i][j] %= m;
      }
    }
  }
  ret
}

fn pow(
  a: Vec<Vec<usize>>,
  n: usize,
  m: usize,
) -> usize {
  let mut a = a.clone();
  let mut n = n-1;
  let mut ret = vec![
    vec![1, 0],
    vec![0, 1],
  ];
  while n > 0 {
    if n & 1 > 0 {
      ret = mul(&a, &ret, m);
    }
    a = mul(&a, &a, m);
    n >>= 1;
  }
  ret[1][0]
}

fn main() {
  let n: usize = read();
  let m: usize = read();
  let a = vec![
    vec![1, 1],
    vec![1, 0],
  ];
  let ans = pow(a, n, m);
  println!("{}", ans);
}