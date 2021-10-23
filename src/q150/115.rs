use proconio::input;

fn main() {
  input! { n: usize }
  let mut grundy = 0;
  for _ in 0..n {
    input! {
      a: usize, b: usize, c: usize,
      m: usize,
      xyz: [(usize, usize, usize); m],
    }
    let mut x = vec![0; m];
    let mut y = vec![0; m];
    let mut z = vec![0; m];
    for i in 0..m {
      x[i] = xyz[i].0;
      y[i] = xyz[i].1;
      z[i] = xyz[i].2;
    }
    x.sort();
    y.sort();
    z.sort();
    grundy ^= x[0] ^ (a - 1 - x[m-1]);
    grundy ^= y[0] ^ (b - 1 - y[m-1]);
    grundy ^= z[0] ^ (c - 1 - z[m-1]);
  }
  let ans = if grundy > 0 { "WIN" } else { "LOSE" };
  println!("{}", ans);
}