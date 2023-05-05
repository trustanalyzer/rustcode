const AMOUNT: usize = 5;

fn main() {
    let coins = vec![1,2,5];
    let res = coin_change(coins);
    dbg!(res);
}

fn coin_change(coins: Vec<usize>) -> usize {

    let n = coins.len();
    if n == 0 {return 0;}

    let mut table = [[0; AMOUNT+1]; 2];
    //dbg!(table);
    //table[0][0] = 1;

    for i in 1..=n {
        for j in 0..=AMOUNT {

            if j == 0 {
                table[i%2][j] = 1;
                continue;
            }

            //dbg!(i, j);
            table[i%2][j] = table[(i-1)%2][j];
            //dbg!(table[i%2][j]);
            if j >= coins[i-1] {
                //dbg!(j - coins[i-1]);
                table[i%2][j] += table[i%2][(j-coins[i-1])];
            }
        }

        //dbg!(table);
    }

    table[n%2][AMOUNT]
}
