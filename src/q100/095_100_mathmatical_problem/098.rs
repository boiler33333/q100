use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
  input! {
    n: usize,
    xs: [usize; n],
  }
  let mut color = vec![0, 0, 0];
  let mut ans = 1;
  for x in xs {
    let cnt = color.iter().cloned().filter(|&c| c == x).collect::<Vec<usize>>().len();
    ans *= cnt;
    ans %= MOD;
    if x == color[0] {
      color[0] += 1;
    } else if x == color[1] {
      color[1] += 1;
    } else if x == color[2] {
      color[2] += 1;
    }
  }
  println!("{}", ans);
}