use std::mem::replace;

fn main() {
    println!("{}", knight_dialer(2));
}

pub fn knight_dialer(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let mut dp = vec![vec![1; 10], vec![0; 10]];
    let moves = vec![vec![4, 6], vec![6, 8], vec![7, 9], vec![4, 8], vec![3, 9, 0], vec![], vec![7, 1, 0], vec![6, 2], vec![1, 3], vec![4, 2]];
    for hoop in 0..n - 1 {
        let ind: usize = (!hoop & 1) as usize;
        let old: usize = (hoop & 1) as usize;
        replace(&mut dp[ind], vec![0; 10]);
        for v in 0..10 {
            for m in &moves[v] {
                let new_val = dp[ind][v] + dp[old][*m];
                replace(&mut dp[ind][v], new_val % MOD);
            }
        }
    }
    let mut ans: i32 = 0;
    let i: usize = (!n & 1) as usize;
    for k in 0..10 {
        ans += dp[i][k];
        ans %= MOD;
    }
    ans
}