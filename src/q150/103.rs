use proconio::input;

fn main() {
  input! {
    n: usize,
    m: i64,
    a: [(i64,i64,i64,i64,i64,i64); n],
  }
  let mut x = vec![];
  let mut y = vec![];
  let mut z = vec![];
  for &(x1,y1,z1,x2,y2,z2) in &a {
    x.push(x1); x.push(x2);
    y.push(y1); y.push(y2);
    z.push(z1); z.push(z2);
  }
  x.sort(); x.dedup();
  y.sort(); y.dedup();
  z.sort(); z.dedup();
  let w = x.len();
  let h = y.len();
  let d = z.len();
  let mut acc = vec![vec![vec![0i64; d]; w]; h];
  for &(x1,y1,z1,x2,y2,z2) in &a {
    let x1 = x.binary_search(&x1).unwrap();
    let y1 = y.binary_search(&y1).unwrap();
    let z1 = z.binary_search(&z1).unwrap();
    let x2 = x.binary_search(&x2).unwrap();
    let y2 = y.binary_search(&y2).unwrap();
    let z2 = z.binary_search(&z2).unwrap();
    acc[y1][x1][z1] += 1; acc[y2][x1][z1] -= 1;
    acc[y1][x2][z1] -= 1; acc[y2][x2][z1] += 1;
    acc[y1][x1][z2] -= 1; acc[y2][x1][z2] += 1;
    acc[y1][x2][z2] += 1; acc[y2][x2][z2] -= 1;
  }
  for i in 1..h {
    for j in 0..w {
      for k in 0..d {
        acc[i][j][k] += acc[i-1][j][k];
      }
    }
  }
  for i in 0..h {
    for j in 1..w {
      for k in 0..d {
        acc[i][j][k] += acc[i][j-1][k];
      }
    }
  }
  for i in 0..h {
    for j in 0..w {
      for k in 1..d {
        acc[i][j][k] += acc[i][j][k-1];
      }
    }
  }
  let mut ans = 0;
  for i in 0..h-1 {
    for j in 0..w-1 {
      for k in 0..d-1 {
        if acc[i][j][k] >= m {
          let y1 = y[i]; let y2 = y[i+1];
          let x1 = x[j]; let x2 = x[j+1];
          let z1 = z[k]; let z2 = z[k+1];
          ans += (x2-x1) * (y2-y1) * (z2-z1);
        }
      }
    }
  }
  println!("{}", ans);
}