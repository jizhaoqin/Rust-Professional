pub fn convert_base(num_str: &str, to_base: u32) -> String {
    // TODO: 这里写逻辑
    // 1. 将输入的 `num_str` 从 `from_base` 解析为 10 进制数
    let parts: Vec<&str> = num_str.split('(').collect();
    let number_str = parts[0];
    let from_base = parts[1].trim_end_matches(')').parse::<u32>().unwrap();
    let decimal_value = u32::from_str_radix(number_str, from_base).unwrap();

    // 2. 将 10 进制数转换为 `to_base`
    let mut result = String::new();
    let mut num = decimal_value;
    while num > 0 {
        let digit = num % to_base;
        let ch = if digit < 10 {
            (b'0' + digit as u8) as char // 0-9
        } else {
            (b'a' + (digit - 10) as u8) as char // A-F for bases > 10
        };
        result.insert(0, ch);
        num /= to_base;
    }

    result
}
