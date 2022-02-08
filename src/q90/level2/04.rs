use proconio::input;

fn main() {
  input! {
    h: usize,
    w: usize,
    a: [[usize; w]; h],
  }
  let mut y = vec![0; h];
  let mut x = vec![0; w];
  for i in 0..h {
    y[i] = a[i].iter().sum();
  }
  for j in 0..w {
    for i in 0..h {
      x[j] += a[i][j];
    }
  }
  for i in 0..h {
    for j in 0..w {
      if j > 0 {
        print!(" ");
      }
      print!("{}", y[i] + x[j] - a[i][j]);
    }
    println!();
  }
}