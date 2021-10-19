use proconio::input;

fn matrix_product(
  k: usize,
  x: &Vec<Vec<i64>>,
  y: &Vec<Vec<i64>>,
) -> Vec<Vec<i64>> {
  let mut ret = vec![vec![0; k]; k];
  for i in 0..k {
    for j in 0..k {
      for l in 0..k {
        ret[i][j] ^= x[i][l] & y[l][j];
      }
    }
  }
  ret
}

fn matrix_power(
  k: usize,
  x: &Vec<Vec<i64>>,
  n: usize,
) -> Vec<Vec<i64>> {
  let mut ret = vec![vec![0; k]; k];
  for i in 0..k {
    ret[i][i] = -1;
  }
  let mut x = x.clone();
  let mut n = n;
  while n > 0 {
    if n & 1 > 0 {
      ret = matrix_product(k, &ret, &x);
    }
    x = matrix_product(k, &x, &x);
    n >>= 1;
  }
  ret
}

fn main() {
  input! {
    k: usize, m: usize,
    a: [i64; k],
    c: [i64; k],
  }
  let mut x = vec![vec![0; k]; k];
  x[0] = c;
  if m <= k {
    println!("{}", a[m-1]);
    return;
  }
  for i in 1..k {
    x[i][i-1] = -1;
  }
  let x = matrix_power(k, &x, m-k);
  let mut ans = 0;
  for i in 0..k {
    ans ^= x[0][i] & a[k-1-i];
  }
  println!("{}", ans);
}