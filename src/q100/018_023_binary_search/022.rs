use proconio::input;

fn f(x: f64, p: f64) -> f64 {
  x + 2.0f64.powf(-x/1.5) * p
}

fn main() {
  input! { p: f64 }
  let mut left = 0.0;
  let mut right = 1e18;
  while right - left > 1e-8 {
    let ml = (2.0 * left + right) / 3.0;
    let mr = (left + 2.0 * right) / 3.0;
    if f(ml, p) < f(mr, p) {
      right = mr;
    } else {
      left = ml;
    }
  }
  println!("{:.10}", f(left, p));
}