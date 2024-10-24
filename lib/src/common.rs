use rinex::prelude::{Constellation, Observable, SV};

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

/// Converts the satellite vehicle (SV) constellation type to a corresponding `u16` value.
///
/// The mapping is as follows:
/// - `Constellation::GPS` => 1
/// - `Constellation::Glonass` => 2
/// - `Constellation::Galileo` => 3
/// - `Constellation::BeiDou` => 4
/// - `Constellation::QZSS` => 5
/// - `Constellation::IRNSS` => 6
/// - Any other constellation type => 7
///
/// # Arguments
///
/// * `sv` - A reference to a satellite vehicle (SV) which contains the constellation type.
///
/// # Returns
///
/// A `u16` value representing the constellation type.
pub fn sv_to_u16(sv: &SV) -> u16 {
    let leading: u16 = match sv.constellation {
        Constellation::GPS => 1,
        Constellation::Glonass => 2,
        Constellation::Galileo => 3,
        Constellation::BeiDou => 4,
        Constellation::QZSS => 5,
        Constellation::IRNSS => 6,
        _ => 7,
    };
    leading * 100 + sv.prn as u16
}

/// Returns the name of the observable field.
///
/// # Arguments
///
/// * `observable` - An observable type.
///
/// # Returns
///
/// An optional string slice containing the name of the observable field. If the observable
/// type is one of phase, doppler, ssi or pseudo range, return the name, else `None` is returned.

#[inline]
pub fn get_observable_field_name(observable: &Observable) -> Option<&str> {
    match observable {
        Observable::Phase(name) => Some(name),
        Observable::Doppler(name) => Some(name),
        Observable::SSI(name) => Some(name),
        Observable::PseudoRange(name) => Some(name),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use rinex::prelude::{Constellation, Observable, SV};

    use crate::common::{get_observable_field_name, sv_to_u16};

    #[test]
    fn test_get_observable_field_name() {
        assert_eq!(
            get_observable_field_name(&Observable::PseudoRange("C1C".to_string())),
            Some("C1C")
        );
        assert_eq!(
            get_observable_field_name(&Observable::Phase("L1C".to_string())),
            Some("L1C")
        );
    }

    #[test]
    fn test_sv_to_u16() {
        let sv_gps = SV {
            constellation: Constellation::GPS,
            prn: 1,
        };
        assert_eq!(sv_to_u16(&sv_gps), 101);

        let sv_galileo = SV {
            constellation: Constellation::Galileo,
            prn: 2,
        };
        assert_eq!(sv_to_u16(&sv_galileo), 302);

        // Add more test cases for other constellations
        let sv_nsas = SV {
            constellation: Constellation::NSAS,
            prn: 24,
        };

        assert_eq!(sv_to_u16(&sv_nsas), 724);

        let sv_compass = SV {
            constellation: Constellation::BeiDou,
            prn: 28,
        };
        assert_eq!(sv_to_u16(&sv_compass), 428);

        let sv_irnss = SV {
            constellation: Constellation::IRNSS,
            prn: 7,
        };
        assert_eq!(sv_to_u16(&sv_irnss), 607);

        let span = SV {
            constellation: Constellation::SPAN,
            prn: 9,
        };
        assert_eq!(sv_to_u16(&span), 709);
    }
}
