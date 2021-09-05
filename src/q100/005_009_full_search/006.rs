use proconio::input;

fn main() {
  input! {
    _: usize,
    s: String
  }
  let s: Vec<usize> = s.bytes().map(|b| (b - 48) as usize).collect();
  let mut ans = 0;
  for i in 0..1000 {
    let pin = vec![i/100, (i/10)%10, i%10];
    let mut j = 0;
    for c in &s {
      if *c == pin[j] {
        j += 1;
      }
      if j == 3 {
        break;
      }
    }
    if j == 3 {
      ans += 1;
    }
  }
  println!("{}", ans);
}