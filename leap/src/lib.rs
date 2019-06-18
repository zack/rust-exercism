pub fn is_leap_year(year: u64) -> bool {
    let div_by_4 = year % 4 == 0;
    let div_by_100 = year % 100 == 0;
    let div_by_400 = year % 400 == 0;

    div_by_4 && !div_by_100 || div_by_400
}
