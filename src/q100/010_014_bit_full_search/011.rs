use proconio::input;

fn main() {
  input! { n: usize, m: usize }
  let mut switches: Vec<Vec<i64>> = Vec::new();
  for _ in 0..m {
    input! { k: usize, s: [i64; k] }
    switches.push(s)
  }
  input! { p: [usize; m] }
  let mut ans = 0;
  for status in 0..1<<n {
    let mut ok = true;
    for i in 0..m {
      let mut cnt = 0;
      for &s in &switches[i] {
        if status & 1 << s-1 != 0 {
          cnt += 1;
        }
      }
      if cnt % 2 != p[i] {
        ok = false;
      }
    }
    if ok {
      ans += 1;
    }
  }
  println!("{}", ans);
}