// https://www.hackerrank.com/challenges/day-of-the-programmer/problem?isFullScreen=true
fn main() {

}

fn day_of_programmer(year: i32) -> String {
    let february_days = get_days_in_february(year);
    let jan_to_aug_days = 31 + february_days + 31 + 30 + 31 + 30 + 31 + 31;
    let september_days = 256 - jan_to_aug_days;
    format!("{:02}.{:02}.{}", september_days, 9, year)
}

fn get_days_in_february(year: i32) -> i32 {
    if year == 1918 {
        return 15;
    } else if is_leap_year(year) {
        return 29;
    }
    28
}

fn is_leap_year(year: i32) -> bool {
    if year < 1918 {
        return year % 4 == 0;
    }
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}
