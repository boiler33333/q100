use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [[usize; n]; n],
  }
  let mut needable = vec![vec![true; n]; n];
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
          needable[i][j] = false;
        }
      }
    }
  }
  let mut ans = 0;
  for y in 0..n {
    for x in 0..n {
      if needable[y][x] {
        ans += a[y][x];
      }
    }
  }
  println!("{}", ans/2);
}