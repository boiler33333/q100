use proconio::input;

const PI: f64 = 3.14159265358979323846264338327950288;

fn main() {
  input! { a: f64, b: f64, x: f64 }
  let s = x / a;
  let rad = if s < a * b / 2.0 {
    // s = a2 * b / 2.0（三角形）
    let a2 = 2.0 * s / b;
    (b/a2).atan()
  } else {
    // s = (b2 + b) * a / 2.0（台形）
    let b2 = (2.0 * s / a) - b;
    ((b - b2)/a).atan()
  };
  // rad = ans / 180.0 * PI
  let ans = rad * 180.0 / PI;
  println!("{:.10}", ans);
}