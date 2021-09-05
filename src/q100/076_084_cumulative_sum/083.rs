use proconio::input;
use std::cmp::min;

fn main() {
  input! {
    n: usize,      //都市の数
    m: usize,      //日数
    p: [usize; m], //行き先
    abc: [(i64, i64, i64); n-1], //紙での運賃, カードでの運賃、カード代
  }
  let mut acc: Vec<i64> = vec![0; n];
  for i in 0..m-1 {
    if p[i+1] > p[i] {
      acc[p[i]-1] += 1;
      acc[p[i+1]-1] -= 1;
    } else {
      acc[p[i+1]-1] += 1;
      acc[p[i]-1] -= 1;
    }
  }
  for i in 0..n-1 {
    acc[i+1] += acc[i];
  }
  let mut ans = 0;
  for i in 0..n-1 {
    let (a, b, c) = abc[i];
    let paper = a * acc[i];
    let card = b * acc[i] + c;
    ans += min(paper, card);
  }
  println!("{}", ans);
}