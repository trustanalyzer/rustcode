
fn main() {
    let sum = count_vowels_permutations(10);
    dbg!(sum);
}

fn count_vowels_permutations(n: usize) -> usize {
    let mut dp = [[0usize; 5]; 2];
    for i in 1usize..n+1 {
        if i == 1 {
            for j in 0usize..5 {
                dp[i][j] = 1;
            }
            continue;
        }

        dp[i%2][0] = dp[(i-1)%2][1] + dp[(i-1)%2][2] + dp[(i-1)%2][4]; // a
        dp[i%2][1] = dp[(i-1)%2][0] + dp[(i-1)%2][2]; // e
        dp[i%2][2] = dp[(i-1)%2][1] + dp[(i-1)%2][3]; // i
        dp[i%2][3] = dp[(i-1)%2][2]; // o
        dp[i%2][4] = dp[(i-1)%2][2] + dp[(i-1)%2][3]; // u

        dbg!(dp[n%2]);
    }

    let sum: usize = dp[n%2].into_iter().sum();
    sum
}
