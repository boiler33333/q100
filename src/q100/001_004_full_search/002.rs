use proconio::input;

fn main() {
  input! { n: usize }
  let mut ans = 0;
  for i in 1..=n {
    let mut cnt = 0;
    for j in 1..=i {
      if i % j == 0 && j % 2 == 1{
        cnt += 1;
      }
    }
    if cnt == 8 {
      ans += 1;
    }
  }
  println!("{}", ans);
}