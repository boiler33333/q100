use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a-1].push(b-1);
        g[b-1].push(a-1);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut cnt = 0;
        for &j in &g[i] {
            if j < i {
                cnt += 1;
            }
        }
        if cnt == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}