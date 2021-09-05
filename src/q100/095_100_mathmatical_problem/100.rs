use proconio::input;

fn main() {
  input! { n: usize }
  let mut m = 1;
  loop {
    if m * (m - 1) == 2 * n {
      break;
    }
    if m * (m - 1) > 2 * n {
      println!("No");
      return;
    }
    m += 1;
  }

  let mut k = 1;
  let mut a = vec![vec![]; m];
  for i in 0..m {
    for j in i+1..m {
      a[i].push(k);
      a[j].push(k);
      k += 1;
    }
  }

  println!("Yes");
  println!("{}", m);
  for xs in a {
    print!("{}", m - 1);
    for x in xs {
      print!(" {}", x)
    }
    println!();
  }
}