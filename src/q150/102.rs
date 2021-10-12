use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }
  let mut b = a.clone();
  b.sort();
  b.dedup();
  for i in 0..n {
    let j = b.binary_search(&a[i]).unwrap();
    println!("{}", j);
  }
}