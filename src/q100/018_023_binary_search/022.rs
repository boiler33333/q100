use proconio::input;

fn f(x: f64, p: f64) -> f64 {
  x + p * 2f64.powf(-x/1.5)
}

fn main() {
  input! { p: f64 }
  let mut low = 0.0;
  let mut high = 1e18;
  while high - low > 1e-8 {
    let x1 = (2.0 * low + 1.0 * high) / 3.0;
    let x2 = (1.0 * low + 2.0 * high) / 3.0;
    if f(x1, p) < f(x2, p) {
      high = x2;
    } else {
      low = x1;
    }
  }
  let ans = f(low, p);
  println!("{:.10}", ans);
}