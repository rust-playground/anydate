//! Date parsing functions
use crate::errors::Error;
use chrono::NaiveDate;

/// Attempts to parse the provided string into a `NaiveDate`.
///
/// # Errors
/// Will return `Err` when an invalid or unsupported `Date` format is provided.
#[inline]
pub fn parse(s: &str) -> Result<NaiveDate, Error> {
    match s.get(..1) {
        None => Err(Error::InvalidDate),
        Some(c) => {
            if c.as_bytes()[0].is_ascii_digit() {
                parse_unknown_alpha(s)
            } else {
                parse_with_alpha(s)
            }
        }
    }
}

pub(crate) fn parse_unknown_alpha(s: &str) -> Result<NaiveDate, Error> {
    parse_naive_dates(s)
}

pub(crate) fn parse_with_alpha(s: &str) -> Result<NaiveDate, Error> {
    parse_naive_dates_replace(s)
}

fn parse_naive_dates(s: &str) -> Result<NaiveDate, Error> {
    // Date parse formats
    const PARSE_FORMATS: &[&str] = &[
        "%Y-%m-%d",
        "%Y/%m/%d",
        "%Y.%m.%d",
        "%m/%d/%y",
        "%m/%d/%Y",
        "%m.%d.%y",
        "%m.%d.%Y",
        "%Y-%b-%d",
        "%d %B %y",
        "%d %B %Y",
        "%Y年%m月%d日",
    ];
    PARSE_FORMATS
        .iter()
        .map(|fmt| NaiveDate::parse_from_str(s, fmt))
        .find_map(Result::ok)
        .map_or_else(|| Err(Error::InvalidDate), Ok)
}

fn parse_naive_dates_replace(s: &str) -> Result<NaiveDate, Error> {
    // Date parse formats
    const PARSE_FORMATS: &[&str] = &[
        "%B %d %y",
        "%B %d %Y",
        "%A %B %eth %Y",
        "%A %B %est %Y",
        "%A %B %end %Y",
        "%A %B %erd %Y",
    ];
    let s = s.replace([',', '.'], "");
    PARSE_FORMATS
        .iter()
        .map(|fmt| NaiveDate::parse_from_str(&s, fmt))
        .find_map(Result::ok)
        .map_or_else(|| Err(Error::InvalidDate), Ok)
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    use super::*;
    use chrono::NaiveTime;

    #[test]
    fn dates() -> Result<(), Box<dyn std::error::Error>> {
        for (input, expected) in &[
            // yyyy-mm-dd
            ("2021-02-21", 1613865600000000000),
            // yyyy-mon-dd
            ("2021-Feb-21", 1613865600000000000),
            // Mon dd, yyyy
            ("May 25, 2021", 1621900800000000000),
            ("oct 7, 1970", 24105600000000000),
            ("oct 7, 70", 24105600000000000),
            ("oct. 7, 1970", 24105600000000000),
            ("oct. 7, 70", 24105600000000000),
            ("October 7, 1970", 24105600000000000),
            // dd Mon yyyy
            ("7 oct 70", 24105600000000000),
            ("7 oct 1970", 24105600000000000),
            ("03 February 2013", 1359849600000000000),
            ("1 July 2013", 1372636800000000000),
            // mm/dd/yyyy
            ("3/31/2014", 1396224000000000000),
            ("03/31/2014", 1396224000000000000),
            ("08/21/71", 51580800000000000),
            ("8/1/71", 49852800000000000),
            // yyyy/mm/dd
            ("2014/3/31", 1396224000000000000),
            ("2014/03/31", 1396224000000000000),
            // mm.dd.yyyy
            ("3.31.2014", 1396224000000000000),
            ("03.31.2014", 1396224000000000000),
            ("08.21.71", 51580800000000000),
            // yyyy.mm.dd
            ("2014.03.30", 1396137600000000000),
            // chinese yyyy mm dd
            ("2014年04月08日", 1396915200000000000),
        ] {
            assert_eq!(
                *expected,
                parse(input)?
                    .and_time(NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap())
                    .timestamp_nanos_opt()
                    .unwrap()
            );
        }
        Ok(())
    }
}
