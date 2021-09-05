use proconio::input;
 
fn calc(x: f64, p: f64) -> f64 {
  x + 2.0f64.powf(-x/1.5) * p
}
 
fn main() {
  input! { p: f64 }
  let mut left = 0.0;
  let mut right = 10.0f64.powi(18);
  for _ in 0..100000 {
    let ml = (2.0 * left + right) / 3.0;
    let mr = (left + 2.0 * right) / 3.0;
    if calc(ml, p) < calc(mr, p) {
      right = mr;
    } else {
      left = ml;
    }
  }
  println!("{}", calc(left, p));
}