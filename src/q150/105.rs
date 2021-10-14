/* -----------------------------------------------
 * input 
 * ----------------------------------------------- */

 macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
      let mut iter = $s.split_whitespace();
      input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
      let s = {
          use std::io::Read;
          let mut s = String::new();
          std::io::stdin().read_to_string(&mut s).unwrap();
          s
      };
      let mut iter = s.split_whitespace();
      input_inner!{iter, $($r)*}
  };
}

macro_rules! input_inner {
  ($iter:expr) => {};
  ($iter:expr, ) => {};
  ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
      let $var = read_value!($iter, $t);
      input_inner!{$iter $($r)*}
  };
}

macro_rules! read_value {
  ($iter:expr, ( $($t:tt),* )) => {
      ( $(read_value!($iter, $t)),* )
  };
  ($iter:expr, [ $t:tt ; $len:expr ]) => {
      (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  };
  ($iter:expr, chars) => {
      read_value!($iter, String).chars().collect::<Vec<char>>()
  };
  ($iter:expr, usize1) => {
      read_value!($iter, usize) - 1
  };
  ($iter:expr, $t:ty) => {
      $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

/* -----------------------------------------------
 * binary search 
 * ----------------------------------------------- */

use std::cmp::Ordering::{ Less, Equal, Greater };

pub trait BinarySearch<T> {
  fn lower_bound(&self, x: &T) -> usize;
  fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
  fn lower_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less => left = mid + 1,
        Equal | Greater => right = mid,
      }
    }
    left
  }

  fn upper_bound(&self, x: &T) -> usize {
    let mut left = 0;
    let mut right = self.len();
    while left < right {
      let mid = (left + right) / 2;
      let ord = self[mid].cmp(x);
      match ord {
        Less | Equal => left = mid + 1,
        Greater => right = mid,
      }
    }
    left
  }
}

/* -----------------------------------------------
 * main
 * ----------------------------------------------- */

fn main() {
  input! {
    n: usize,
    k: usize,
    l: i64,
    r: i64,
    a: [i64; n],
  }
  let ans = solve(n, k, l, r, &a);
  println!("{}", ans);
}

fn solve(
  n: usize,
  k: usize,
  l: i64,
  r: i64,
  a: &[i64],
) -> usize {
  let mut al: Vec<(usize, i64)> = vec![];
  for state in 0..1<<n/2 {
    let mut cnt = 0;
    let mut sum = 0;
    for i in 0..n/2 {
      if state & 1 << i > 0 {
        cnt += 1;
        sum += a[i];
      }
    }
    al.push((cnt, sum));
  }

  let mut ar: Vec<(usize, i64)> = vec![];
  let m = n - n/2;
  for state in 0..1<<m {
    let mut cnt = 0;
    let mut sum = 0;
    for i in 0..m {
      if state & 1 << i > 0 {
        cnt += 1;
        sum += a[i + n/2];
      }
    }
    ar.push((cnt, sum));
  }

  ar.sort();

  let mut ret = 0;
  for &(cnt, sum) in &al {
    let ll = ar.lower_bound(&(k - cnt, l - sum));
    let rr = ar.upper_bound(&(k - cnt, r - sum));
    ret += rr - ll;
  }
  ret
}

/* -----------------------------------------------
 * test
 * ----------------------------------------------- */

#[test]
fn test_solve_1() {
  let (n, k, l, r) = (2, 2, 1, 9);
  let a = vec![5, 1];
  let got = solve(n, k, l, r, &a);
  let want = 1;
  assert_eq!(got, want);
}

#[test]
fn test_solve_2() {
  let (n, k, l, r) = (5, 2, 7, 19);
  let a = vec![3, 5, 4, 2, 2];
  let got = solve(n, k, l, r, &a);
  let want = 5;
  assert_eq!(got, want);
}