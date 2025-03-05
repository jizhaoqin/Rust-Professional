use rand::Rng;

/// 计算 (base^exp) % mod 高效幂取模
fn mod_pow(mut base: u128, mut exp: u128, modu: u128) -> u128 {
    let mut result = 1;
    base %= modu;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modu;
        }
        base = (base * base) % modu;
        exp /= 2;
    }
    result
}

/// Miller-Rabin 素性测试
fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    let bases = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229,
    ];

    for &a in bases.iter() {
        if a >= n {
            continue;
        }
        // print!("{}, ", a);
        let mut x = mod_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut continue_outer = false;
        for _ in 0..r - 1 {
            x = (x * x) % n;
            if x == n - 1 {
                continue_outer = true;
                break;
            }
        }
        if !continue_outer {
            return false;
        }
    }
    true
}

/// 计算 GCD
fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Pollard's Rho 算法快速因子分解
fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }

    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(2..n);
    let mut y = x;
    let c = rng.gen_range(1..n);
    let mut d = 1;

    while d == 1 {
        x = (mod_pow(x, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        y = (mod_pow(y, 2, n) + c) % n;
        d = gcd((x as i128 - y as i128).unsigned_abs(), n);
    }
    d
}

pub fn find_max_prime_factor(mut number: u128) -> u128 {
    let mut max_prime = 1;

    // 处理2的因子
    while number % 2 == 0 {
        max_prime = 2;
        number /= 2;
    }

    // 处理100次3以上的因子
    let mut factor = 3;
    for _ in 1..=100 {
        while number % factor == 0 {
            max_prime = factor;
            number /= factor;
        }
        factor += 2;
    }

    // Pollard's Rho 算法快速因子分解
    while number > 1 {
        // 判断分解后是否为prime factor
        if is_prime(number) {
            return number;
        }
        let factor = pollard_rho(number);
        while number % factor == 0 {
            number /= factor;
            max_prime = factor;
        }
    }

    max_prime
}
