fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    (3..=sqrt_n).step_by(2).all(|i| n % i != 0)
}

fn is_goldbach_number(n: u64) -> bool {
    for square in 1.. {
        let double_square = 2 * square * square;
        if double_square >= n {
            break;
        }
        let remainder = n - double_square;
        if is_prime(remainder) {
            return true;
        }
    }
    false
}

pub fn goldbach_conjecture() -> String {
    let count = 2;
    let mut result = Vec::new();
    let mut num = 9; // 最小的奇合数
    while result.len() < count {
        if !is_prime(num) && !is_goldbach_number(num) {
            result.push(num);
        }
        num += 2; // 只检查奇数
    }
    result
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
