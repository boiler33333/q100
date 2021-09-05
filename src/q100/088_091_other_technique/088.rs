use proconio::input;

fn main() {
  input! {
    n: usize,
    cs: [usize; n],
  }
  let mut xs: Vec<(usize, usize)> = vec![];
  for (i, &c) in cs.iter().enumerate() {
    if i % 2 == 0 {
      match xs.last_mut() {
        Some(x) if x.0 == c => x.1 += 1,
        _ => xs.push((c, 1)),
      }
    } else {
      match xs.pop() {
        Some(x) if x.0 == c => xs.push((c, x.1 + 1)),
        Some(x) => match xs.last_mut() {
          Some(y) => y.1 += x.1 + 1,
          None => xs.push((c, x.1 + 1)),
        }
        None => unreachable!(),
      }
    }
  }
  let ans: usize = xs.iter().filter(|(x, _)| *x == 0).map(|(_, cnt)| cnt).sum();
  println!("{}", ans);
}