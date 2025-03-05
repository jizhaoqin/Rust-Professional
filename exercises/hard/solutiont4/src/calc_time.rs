// 是否是闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// 获取某年某月的天数
fn get_days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

// 计算某年某月某日是本年的第几天
fn day_of_year(year: i32, month: i32, day: i32) -> i32 {
    let mut days = 0;
    for m in 1..month {
        days += get_days_in_month(year, m);
    }
    days + day
}

// 计算某年某月某日是星期几（2025-01-01 是周三）
fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    let mut total_days = 0;
    for y in 2025..year {
        total_days += if is_leap_year(y) { 366 } else { 365 };
    }
    total_days += day_of_year(year, month, day);
    total_days % 7 + 2
}

// 计算当前周是第几周
fn week_of_year(year: i32, month: i32, day: i32) -> i32 {
    let first_day_of_year = day_of_week(year, 1, 1);
    let day_num = day_of_year(year, month, day);
    let week = ((day_num + first_day_of_year - 1 - 1) / 7) + 1;
    week % 52
}

// 计算距离农历新年（正月初一）还有多少天（假设是 2024-02-10）
fn days_to_chinese_new_year(year: i32, month: i32, day: i32) -> i32 {
    let next_cny = if year == 2025 && month == 1 && day <= 29 {
        (2025, 1, 29)
    } else {
        (2026, 2, 17)
    };
    let days_current = day_of_year(year, month, day);
    let days_cny = day_of_year(next_cny.0, next_cny.1, next_cny.2);

    if year == next_cny.0 {
        days_cny - days_current
    } else {
        (if is_leap_year(year) { 366 } else { 365 }) - days_current + days_cny
    }
}

// 计算距离下一次 A 股开盘还有多少天（A 股周一到周五开盘，假设无假期）
fn days_to_next_stock_open(year: i32, month: i32, day: i32) -> i32 {
    let this_day = day_of_week(year, month, day);
    match this_day {
        5 => return 2,
        6 => return 1,
        _ => {}
    }

    match (month, day) {
        (1, x) if x >= 28 => return 7 - (x - 28),
        (2, x) if x <= 4 => return 4 - x,
        (12, 31) => return 1,
        (5, 1) => return 4,
        _ => {}
    }
    0
}

pub fn time_info(time: &str) -> String {
    let parts: Vec<i32> = time.split('-').map(|s| s.parse().unwrap()).collect();
    let (year, month, day) = (parts[0], parts[1], parts[2]);

    let week_num = week_of_year(year, month, day);
    let weekday = day_of_week(year, month, day);
    let day_of_year = day_of_year(year, month, day);
    let days_left = if is_leap_year(year) { 366 } else { 365 } - day_of_year;
    let days_to_cny = days_to_chinese_new_year(year, month, day);
    let days_to_stock_open = days_to_next_stock_open(year, month, day);

    format!(
        "{},{},{},{},{},{}",
        week_num, weekday, day_of_year, days_left, days_to_cny, days_to_stock_open
    )
}
