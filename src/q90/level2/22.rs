use proconio::input;

fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn main() {
    input! { a: u128, b: u128, c: u128 }
    let d = gcd(gcd(a, b), c);
    let mut ans = 0;
    ans += a/d - 1;
    ans += b/d - 1;
    ans += c/d - 1;
    println!("{}", ans);
}