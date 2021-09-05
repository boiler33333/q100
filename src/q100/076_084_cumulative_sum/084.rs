use proconio::input;

fn main() {
  input! {
    n: usize, //正三角形の一辺に並んでいる釘の本
    m: usize, //輪ゴムの数
    abx: [(usize, usize, usize); m], //3本の釘 (Ai, Bi), (Ai + Xi, Bi), (Ai + Xi, Bi + Xi)
  }
  let mut acc = vec![vec![0; n+3]; n+3];
  for (a, b, x) in abx {
    acc[a][b] += 1;
    acc[a][b+1] -= 1;
    acc[a+x+1][b] -= 1;
    acc[a+x+1][b+x+2] += 1;
    acc[a+x+2][b+1] += 1;
    acc[a+x+2][b+x+2] -= 1;
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a][b-1];
    }
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a-1][b];
    }
  }
  for a in 1..=n+2 {
    for b in 1..=n+2 {
      acc[a][b] += acc[a-1][b-1];
    }
  }
  let mut ans = 0;
  for a in 1..=n {
    for b in 1..=a {
      if acc[a][b] > 0 {
        ans += 1;
      }
    }
  }
  println!("{}", ans);
}