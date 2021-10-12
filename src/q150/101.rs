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
 * main 
 * ----------------------------------------------- */
fn main() {
  input! {
    n: usize,
    a: [(i64, i64, i64, i64); n],
  }
  let ans = solve(n, &a);
  println!("{}", ans);
}

fn solve(_: usize, a: &[(i64, i64, i64, i64)]) -> i64 {
  let mut x = vec![];
  let mut y = vec![];
  for &(x1, y1, x2, y2) in a {
    x.push(x1);
    y.push(y1);
    x.push(x2);
    y.push(y2);
  }
  x.sort();
  x.dedup();
  y.sort();
  y.dedup();

  let h = y.len();
  let w = x.len();
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
      if table[i][j] {
        let y1 = y[i];
        let y2 = y[i+1];
        let x1 = x[j];
        let x2 = x[j+1];
        ret += (y2 - y1) * (x2 - x1);
      }
    }
  }
  ret
}

#[test]
fn test_solve_1() {
  let n = 2;
  let a = vec![(0, 0, 3, 4), (1, 2, 4, 3)];
  let got = solve(n, &a);
  assert_eq!(got, 13);
}

#[test]
fn test_solve_2() {
  let n = 3;
  let a = vec![(1, 1, 2, 5), (2, 1, 5, 2), (1, 2, 2, 5)];
  let got = solve(n, &a);
  assert_eq!(got, 7);
}

#[test]
fn test_solve_3() {
  let n = 4;
  let a = vec![(0, 0, 3, 1), (0, 0, 1, 3), (0, 2, 3, 3), (2, 0, 3, 3)];
  let got = solve(n, &a);
  assert_eq!(got, 8);
}