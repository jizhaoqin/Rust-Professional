pub fn retire_time(time: &str, retire_type: &str) -> String {
    let (birth_year, birth_month) = {
        let parts: Vec<&str> = time.split('-').collect();
        (
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        )
    };

    let (start_year, old_retire_age, max_delayed_month) = match retire_type {
        "原法定退休年龄55周岁女职工" => (1970, 55, 36),
        "原法定退休年龄50周岁女职工" => (1975, 50, 60),
        "男职工" => (1965, 60, 36),
        _ => panic!("Unknown type"),
    };

    let mut retire_year = birth_year + old_retire_age;
    let mut retire_month = birth_month;
    let mut retire_age = old_retire_age as f32;
    let mut delayed_month = 0;

    if birth_year >= start_year {
        let months = (birth_year - start_year) * 12 + (birth_month - 1);
        delayed_month = (months - 1) / 4 + 1;
        delayed_month = delayed_month.min(max_delayed_month);
        retire_year += (delayed_month + birth_month - 1) / 12;
        retire_month = (delayed_month + birth_month - 1) % 12 + 1;
        retire_age += delayed_month as f32 / 12_f32;
    }

    if delayed_month % 12 == 0 {
        format!(
            "{:04}-{:02},{:.0},{}",
            retire_year, retire_month, retire_age, delayed_month,
        )
    } else {
        format!(
            "{:04}-{:02},{:.2},{}",
            retire_year, retire_month, retire_age, delayed_month,
        )
    }
}
