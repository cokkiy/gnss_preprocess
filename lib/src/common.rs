/// Returns the next day given a year and the day of the year.
///
/// # Arguments
///
/// * `year` - A 16-bit unsigned integer representing the year.
/// * `day_of_year` - A 16-bit unsigned integer representing the day of the year.
///
/// # Returns
///
/// A tuple containing:
/// * The year of the next day.
/// * The day of the year of the next day.
///
/// # Examples
///
/// ```
/// let next_day = get_next_day(2023, 365);
/// assert_eq!(next_day, (2024, 1));
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Notes
///
/// This function accounts for leap years.
///
pub fn get_next_day(year: u16, day_of_year: u16) -> (u16, u16) {
    if is_leap_year(year) {
        if day_of_year == 366 {
            return (year + 1, 1);
        }
    } else if day_of_year == 365 {
        return (year + 1, 1);
    }
    (year, day_of_year + 1)
}

/// Determines if a given year is a leap year. If the year is two digital,
/// it is converted to a four digital year by add 2000.
pub fn is_leap_year(year: u16) -> bool {
    let mut year = year;
    if year < 100 {
        year += 2000;
    }
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
