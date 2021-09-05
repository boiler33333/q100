use proconio::input;

const PI: f64 = 3.14159265358979323846264338327950288;

fn main() {
  input! { a: f64, b: f64, x: f64 }
  let v = a * a * b;
  let theta = if x < v / 2.0 {
    // 水 = 三角形 * 奥行き
    // x = (a2 * b / 2.0) * a
    let a2 = 2.0 * x / (a * b);
    (b/a2).atan()
  } else {
    // 水 = 台形 * 奥行き
    // x = ((b2 + b) * a / 2.0) * a
    let b2 = (2.0 * x) / (a * a) - b;
    ((b-b2)/a).atan()
  };
  println!("{:.10}", theta * 180.0 / PI);
}