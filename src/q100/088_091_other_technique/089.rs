use proconio::input;
use std::cmp::max;

fn main() {
  input! { n: usize, a: [usize; n] }
  let mut xs = vec![0];
  for i in 1..n {
    if a[i] == a[i-1] {
      xs.push(i);
    }
  }
  xs.push(n);
  let m = xs.len();
  if m <= 3 {
    println!("{}", n);
    return;
  }
  let mut ans = 0;
  for j in 3..m {
    ans = max(ans, xs[j] - xs[j-3]);
  }
  println!("{}", ans);
}