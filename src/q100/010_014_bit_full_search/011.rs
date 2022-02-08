use proconio::input;

fn main() {
  input! { n: usize, m: usize }
  let mut s = vec![];
  for _ in 0..m {
    input! { k: usize, si: [usize; k] }
    s.push(si);
  }
  input! { p: [usize; m] }
  let mut ans = 0;
  for state in 0..1<<n {
    let mut ok = true;
    for j in 0..m {
      let mut cnt = 0;
      for &i in &s[j] {
        if state & 1 << (i-1) > 0 {
          cnt += 1;
        }
      }
      if cnt % 2 != p[j] {
        ok = false;
      }
    }
    if ok {
      ans += 1;
    } 
  }
  println!("{}", ans);
}