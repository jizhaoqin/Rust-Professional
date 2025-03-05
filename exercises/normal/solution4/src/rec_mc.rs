pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let coins = [1, 2, 5, 10, 20, 30, 50, 100];
    let mut dp = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins.iter() {
            if i >= coin {
                dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
            }
        }
    }

    if dp[amount as usize] > amount {
        0
    } else {
        dp[amount as usize]
    }
}
