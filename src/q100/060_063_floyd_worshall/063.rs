use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [[usize; n]; n],
  }
  let mut available = vec![vec![true; n]; n];
  for k in 0..n {
    for i in 0..n {
      for j in 0..n {
        if a[i][j] > a[i][k] + a[k][j] {
          println!("-1");
          return;
        }
        if i == k || k == j {
          continue;
        }
        if a[i][j] == a[i][k] + a[k][j] {
          available[i][j] = false;
        }
      }
    }
  }
  let mut ans = 0;
  for i in 0..n {
    for j in 0..n {
      if available[i][j] {
        ans += a[i][j];
      }
    }
  }
  println!("{}", ans/2);
}