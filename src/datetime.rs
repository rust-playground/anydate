//! DateTime parsing functions
use crate::errors::Error;
use chrono::{DateTime, FixedOffset, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc};

/// Attempts to parse the provided string into a DateTime\<FixedOffset\>.
/// Also see [`parse_utc`] for a convenience conversion to DateTime\<Utc\>.
#[inline]
pub fn parse(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    match s.get(..1) {
        None => Err(Error::InvalidDateTime),
        Some(c) => {
            if c.as_bytes()[0].is_ascii_digit() {
                parse_unknown_alpha(s)
            } else {
                parse_with_alpha(s)
            }
        }
    }
}

/// Attempts to parse the provided string into a DateTime\<FixedOffset\> but convert it to a
/// DateTime\<Utc\> prior to returning automatically.
#[inline]
pub fn parse_utc(s: &str) -> Result<DateTime<Utc>, Error> {
    let fdt = parse(s)?;
    Ok(fdt.with_timezone(&Utc))
}

fn parse_unknown_alpha(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    parse_unix_timestamp(s)
        .or_else(|_| parse_rfc3339(s))
        .or_else(|_| parse_rfc2822(s))
        .or_else(|_| parse_is08601(s))
        .or_else(|_| parse_naive_datetime(s))
        .or_else(|_| parse_utc_naive_datetime_unknown_alpha(s))
        .or_else(|_| parse_utc_naive_datetime_replace_str_unknown_alpha(s))
        .or_else(|_| {
            let dt = crate::date::parse_unknown_alpha(s).map_err(|_| Error::InvalidDateTime)?;
            let ndt = NaiveDateTime::new(
                dt,
                NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap_or_default(),
            );
            Ok(Utc.fix().from_utc_datetime(&ndt))
        })
        .or_else(|_: Error| parse_timezone_abbreviation_unknown_alpha(s))
}

fn parse_with_alpha(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    parse_rfc2822(s)
        .or_else(|_| parse_naive_datetime(s))
        .or_else(|_| parse_utc_naive_datetime_alpha_prefix(s))
        .or_else(|_| parse_utc_naive_datetime_replace_str_prefix_alpha(s))
        .or_else(|_| {
            let dt = crate::date::parse_with_alpha(s)
                .map_err(|_| Error::InvalidDateTime)?
                .and_time(NaiveTime::default())
                .and_utc();
            Ok(dt.fixed_offset())
        })
        .or_else(|_: Error| parse_timezone_abbreviation_prefix_alpha(s))
}

fn parse_unix_timestamp(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    if s.len() <= 10 {
        // unix timestamp - seconds
        match s.parse::<i64>() {
            Ok(u) => {
                let utc = Utc.timestamp_opt(u, 0).unwrap();
                Ok(DateTime::from(utc))
            }
            Err(_) => Err(Error::InvalidDateTime),
        }
    } else if s.len() <= 13 {
        // unix timestamp - milliseconds
        match s.parse::<i64>() {
            Ok(u) => {
                let utc = Utc.timestamp_nanos(u * 1_000_000);
                Ok(DateTime::from(utc))
            }
            Err(_) => Err(Error::InvalidDateTime),
        }
    } else if s.len() <= 16 {
        // unix timestamp - microseconds
        match s.parse::<i64>() {
            Ok(u) => {
                let utc = Utc.timestamp_nanos(u * 1_000);
                Ok(DateTime::from(utc))
            }
            Err(_) => Err(Error::InvalidDateTime),
        }
    } else if s.len() <= 19 {
        // unix timestamp - nanoseconds
        match s.parse::<i64>() {
            Ok(u) => {
                let utc = Utc.timestamp_nanos(u);
                Ok(DateTime::from(utc))
            }
            Err(_) => Err(Error::InvalidDateTime),
        }
    } else {
        Err(Error::InvalidDateTime)
    }
}

fn parse_is08601(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    s.parse::<DateTime<FixedOffset>>()
        .map_err(|_| Error::InvalidDateTime)
}

fn parse_rfc3339(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    DateTime::parse_from_rfc3339(s).map_err(|_| Error::InvalidDateTime)
}

fn parse_rfc2822(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    DateTime::parse_from_rfc2822(s).map_err(|_| Error::InvalidDateTime)
}

fn parse_naive_datetime(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    // DateTimes with timezone info
    const PARSE_FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S %z",
        "%Y-%m-%d %H:%M:%S%.f%#z",
        "%Y-%m-%d %H:%M:%S%#z",
        "%Y-%m-%d %H:%M%#z",
    ];
    PARSE_FORMATS
        .iter()
        .map(|fmt| DateTime::parse_from_str(s, fmt))
        .find_map(Result::ok)
        .map_or_else(|| Err(Error::InvalidDateTime), Ok)
}

fn parse_utc_naive_datetime_unknown_alpha(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    // DateTimes without timezone info
    const PARSE_FORMATS: &[&str] = &[
        "%Y-%m-%d %H:%M:%S%.f",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%d %H:%M",
        "%m/%d/%y %H:%M:%S",
        "%m/%d/%y %H:%M",
        "%m/%d/%y %H:%M:%S%.f",
        "%m/%d/%Y %H:%M:%S",
        "%m/%d/%Y %H:%M",
        "%m/%d/%Y %H:%M:%S%.f",
        "%y%m%d %H:%M:%S",
        "%Y/%m/%d %H:%M:%S",
        "%Y/%m/%d %H:%M",
        "%Y/%m/%d %H:%M:%S%.f",
        "%Y-%m-%d %I:%M:%S %P",
        "%Y-%m-%d %I:%M %P",
        "%m/%d/%y %I:%M:%S %P",
        "%m/%d/%y %I:%M %P",
        "%m/%d/%Y %I:%M:%S %P",
        "%m/%d/%Y %I:%M %P",
        "%Y/%m/%d %I:%M:%S %P",
        "%Y/%m/%d %I:%M %P",
        "%Y年%m月%d日%H时%M分%S秒",
    ];
    parse_utc_naive_datetime(s, PARSE_FORMATS)
}

fn parse_utc_naive_datetime_alpha_prefix(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    // DateTimes without timezone info
    const PARSE_FORMATS: &[&str] = &["%A %B %e %T %Y"];
    parse_utc_naive_datetime(s, PARSE_FORMATS)
}

fn parse_utc_naive_datetime_replace_str_unknown_alpha(
    s: &str,
) -> Result<DateTime<FixedOffset>, Error> {
    // DateTimes without timezone info
    const PARSE_FORMATS: &[&str] = &[
        "%d %B %Y %H:%M:%S",
        "%d %B %Y %H:%M",
        "%d %B %Y %H:%M:%S%.f",
        "%d %B %Y %I:%M:%S %P",
        "%d %B %Y %I:%M %P",
    ];
    let s = s.replace(',', "");
    parse_utc_naive_datetime(&s, PARSE_FORMATS)
}

fn parse_utc_naive_datetime_replace_str_prefix_alpha(
    s: &str,
) -> Result<DateTime<FixedOffset>, Error> {
    // DateTimes without timezone info
    const PARSE_FORMATS: &[&str] = &[
        "%B %d %Y %H:%M:%S",
        "%B %d %Y %H:%M",
        "%B %d %Y %I:%M:%S %P",
        "%B %d %Y %I:%M %P",
    ];
    let s = s.replace(',', "");
    parse_utc_naive_datetime(&s, PARSE_FORMATS)
}

fn parse_utc_naive_datetime(s: &str, formats: &[&str]) -> Result<DateTime<FixedOffset>, Error> {
    formats
        .iter()
        .map(|fmt| NaiveDateTime::parse_from_str(s, fmt))
        .find_map(Result::ok)
        .map_or_else(
            || Err(Error::InvalidDateTime),
            |dt| Ok(DateTime::from(dt.and_utc())),
        )
}

// last ditch effort, timezone abbreviation can't 100% relied upon.
//
// It is not possible to reliably convert from an abbreviation to an offset, for example CDT can
// mean either Central Daylight Time (North America) or China Daylight Time.
//
// list sourced from https://www.utctime.net/time-zone-abbreviations
//
fn parse_timezone_abbreviation_unknown_alpha(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    s.rsplit_once(' ').map_or_else(
        || Err(Error::InvalidDateTime),
        |(s, tz)| {
            let offset = parse_offset(tz)?;
            let dt = parse_utc_naive_datetime_unknown_alpha(s)
                .or_else(|_| parse_utc_naive_datetime_replace_str_unknown_alpha(s))?
                - offset;
            Ok(offset.from_utc_datetime(&dt.naive_utc()))
        },
    )
}

// last ditch effort, timezone abbreviation can't 100% relied upon.
//
// It is not possible to reliably convert from an abbreviation to an offset, for example CDT can
// mean either Central Daylight Time (North America) or China Daylight Time.
//
// list sourced from https://www.utctime.net/time-zone-abbreviations
//
fn parse_timezone_abbreviation_prefix_alpha(s: &str) -> Result<DateTime<FixedOffset>, Error> {
    s.rsplit_once(' ').map_or_else(
        || Err(Error::InvalidDateTime),
        |(s, tz)| {
            let offset = parse_offset(tz)?;
            let dt = parse_utc_naive_datetime_alpha_prefix(s)
                .or_else(|_| parse_utc_naive_datetime_replace_str_prefix_alpha(s))?
                - offset;
            Ok(offset.from_utc_datetime(&dt.naive_utc()))
        },
    )
}

fn parse_offset(tz: &str) -> Result<FixedOffset, Error> {
    let offset = match tz.to_uppercase().as_str() {
        "GMT" | "IBST" | "WET" | "Z" | "EGST" => Utc.fix(),
        "BST" | "CET" | "DFT" | "IST" | "MET" | "WAT" | "WEDT" | "WEST" => {
            FixedOffset::east_opt(3600).unwrap_or(Utc.fix())
        }
        "CAT" | "CEDT" | "CEST" | "EET" | "HAEC" | "IST Israel" | "MEST" | "SAST" | "USZ1"
        | "WAST" | "AST Arabia" | "EAT" => FixedOffset::east_opt(2 * 3600).unwrap_or(Utc.fix()),
        "EEDT" | "EEST" | "FET" | "IDT" | "IOT" | "MSK" | "SYOT" => {
            FixedOffset::east_opt(3 * 3600).unwrap_or(Utc.fix())
        }
        "IRST" => FixedOffset::east_opt(3 * 3600 + 1800).unwrap_or(Utc.fix()),
        "AMT Armenia" | "AZT" | "GET" | "GST Gulf" | "MUT" | "RET" | "SAMT" | "SCT" | "VOLT" => {
            FixedOffset::east_opt(4 * 3600).unwrap_or(Utc.fix())
        }
        "AFT" | "IRDT" => FixedOffset::east_opt(4 * 3600 + 1800).unwrap_or(Utc.fix()),
        "HMT" | "MAWT" | "MVT" | "ORAT" | "PKT" | "TFT" | "TJT" | "TMT" | "UZT" | "YEKT" => {
            FixedOffset::east_opt(5 * 3600).unwrap_or(Utc.fix())
        }
        "IST Indian" | "SLST" => FixedOffset::east_opt(5 * 3600 + 1800).unwrap_or(Utc.fix()),
        "NPT" => FixedOffset::east_opt(5 * 3600 + 2700).unwrap_or(Utc.fix()),
        "BDT Bangladesh" | "BIOT" | "BST Bangladesh" | "BTT" | "KGT" | "OMST" | "VOST" => {
            FixedOffset::east_opt(6 * 3600).unwrap_or(Utc.fix())
        }
        "CCT" | "MMT" | "MST Myanmar" => {
            FixedOffset::east_opt(6 * 3600 + 1800).unwrap_or(Utc.fix())
        }
        "CXT" | "DAVT" | "HOVT" | "ICT" | "KRAT" | "THA" | "WIT" => {
            FixedOffset::east_opt(7 * 3600).unwrap_or(Utc.fix())
        }
        "ACT" | "AWST" | "BDT" | "CHOT" | "CIT" | "CST China" | "CT" | "HKT" | "IRKT"
        | "MST Malaysia" | "MYT" | "PST Philippine" | "SGT" | "SST" | "ULAT" | "WST" => {
            FixedOffset::east_opt(8 * 3600).unwrap_or(Utc.fix())
        }
        "CWST" => FixedOffset::east_opt(8 * 3600 + 2700).unwrap_or(Utc.fix()),
        "AWDT" | "EIT" | "JST" | "KST" | "TLT" | "YAKT" => {
            FixedOffset::east_opt(9 * 3600).unwrap_or(Utc.fix())
        }
        "ACST" | "CST Australia Central" => {
            FixedOffset::east_opt(9 * 3600 + 1800).unwrap_or(Utc.fix())
        }
        "AEST" | "ChST" | "CHUT" | "DDUT" | "EST Australia" | "PGT" | "VLAT" => {
            FixedOffset::east_opt(10 * 3600).unwrap_or(Utc.fix())
        }
        "ACDT" | "CST Australia Central Summer" | "LHST" => {
            FixedOffset::east_opt(10 * 3600 + 1800).unwrap_or(Utc.fix())
        }
        "AEDT"
        | "BST Bougainville"
        | "KOST"
        | "LHST Lord Howe Summer"
        | "MIST"
        | "NCT"
        | "PONT"
        | "SAKT"
        | "SBT"
        | "SRET"
        | "VUT"
        | "NFT" => FixedOffset::east_opt(11 * 3600).unwrap_or(Utc.fix()),
        "FJT" | "GILT" | "MAGT" | "MHT" | "NZST" | "PETT" | "TVT" | "WAKT" => {
            FixedOffset::east_opt(12 * 3600).unwrap_or(Utc.fix())
        }
        "CHAST" => FixedOffset::east_opt(12 * 3600 + 2700).unwrap_or(Utc.fix()),
        "NZDT" | "PHOT" | "TKT" | "TOT" => FixedOffset::east_opt(13 * 3600).unwrap_or(Utc.fix()),
        "CHADT" => FixedOffset::east_opt(13 * 3600 + 2700).unwrap_or(Utc.fix()),
        "LINT" => FixedOffset::east_opt(14 * 3600).unwrap_or(Utc.fix()),
        "AZOST" | "CVT" | "EGT" => FixedOffset::west_opt(3600).unwrap_or(Utc.fix()),
        "BRST" | "FNT" | "GST" | "PMDT" | "UYST" => {
            FixedOffset::west_opt(2 * 3600).unwrap_or(Utc.fix())
        }
        "NDT" => FixedOffset::west_opt(2 * 3600 + 1800).unwrap_or(Utc.fix()),
        "ADT"
        | "AMST"
        | "ART"
        | "BRT"
        | "CLST"
        | "FKST"
        | "FKST Falkland Islands Summer"
        | "GFT"
        | "PMST"
        | "PYST"
        | "ROTT"
        | "SRT"
        | "UYT" => FixedOffset::west_opt(3 * 3600).unwrap_or(Utc.fix()),
        "NST" | "NT" => FixedOffset::west_opt(3 * 3600 + 1800).unwrap_or(Utc.fix()),
        "AMT" | "AST" | "BOT" | "CDT Cuba" | "CLT" | "COST" | "ECT" | "EDT" | "FKT" | "GYT"
        | "PYT" => FixedOffset::west_opt(4 * 3600).unwrap_or(Utc.fix()),
        "VET" => FixedOffset::west_opt(4 * 3600 + 1800).unwrap_or(Utc.fix()),
        "ACT Acre" | "CDT" | "COT" | "CST Cuba" | "EASST" | "ECT Ecuador" | "EST" | "PET" => {
            FixedOffset::west_opt(5 * 3600).unwrap_or(Utc.fix())
        }
        "CST" | "EAST" | "GALT" | "MDT" => FixedOffset::west_opt(6 * 3600).unwrap_or(Utc.fix()),
        "MST" | "PDT" => FixedOffset::west_opt(7 * 3600).unwrap_or(Utc.fix()),
        "AKDT" | "CIST" | "PST" => FixedOffset::west_opt(8 * 3600).unwrap_or(Utc.fix()),
        "AKST" | "GAMT" | "GIT" | "HADT" => FixedOffset::west_opt(9 * 3600).unwrap_or(Utc.fix()),
        "MART" | "MIT" => FixedOffset::west_opt(9 * 3600 + 1800).unwrap_or(Utc.fix()),
        "CKT" | "HAST" | "HST" | "TAHT" => FixedOffset::west_opt(10 * 3600).unwrap_or(Utc.fix()),
        "NUT" | "SST Samoa" => FixedOffset::west_opt(11 * 3600).unwrap_or(Utc.fix()),
        "BIT" => FixedOffset::west_opt(12 * 3600).unwrap_or(Utc.fix()),
        _ => return Err(Error::InvalidDateTime),
    };
    Ok(offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_timestamp() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(1636331169, parse_utc("1636331169")?.timestamp());
        assert_eq!(
            1636331272246,
            parse_utc("1636331272246")?.timestamp_millis()
        );
        assert_eq!(
            1636331272246000,
            parse_utc("1636331272246000")?
                .timestamp_nanos_opt()
                .unwrap()
                / 1_000
        );
        assert_eq!(
            1636331290175019000,
            parse_utc("1636331290175019000")?
                .timestamp_nanos_opt()
                .unwrap()
        );
        Ok(())
    }

    #[test]
    fn rfc3339() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            1636331565426737000,
            parse_utc("2021-11-08T00:32:45.426737000Z")?
                .timestamp_nanos_opt()
                .unwrap()
        );
        assert_eq!(
            1636331565426737,
            parse_utc("2021-11-08T00:32:45.426737Z")?
                .timestamp_nanos_opt()
                .unwrap()
                / 1_000
        );
        assert_eq!(
            1636331565426,
            parse_utc("2021-11-08T00:32:45.426Z")?.timestamp_millis()
        );
        assert_eq!(1636331565, parse_utc("2021-11-08T00:32:45Z")?.timestamp());
        Ok(())
    }

    #[test]
    fn rfc2822() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            1636331718000000000,
            parse_utc("Mon, 08 Nov 2021 00:35:18 +0000")?
                .timestamp_nanos_opt()
                .unwrap()
        );
        Ok(())
    }

    #[test]
    fn misc_timestamps() -> Result<(), Box<dyn std::error::Error>> {
        for (input, expected) in &[
            // RFC3339Nano
            ("2021-11-08T00:54:37.059879000Z", 1636332877059879000),
            // RFC2822
            ("Mon, 08 Nov 2021 00:35:18 +0000", 1636331718000000000),
            // Postgres timestamp yyyy-mm-dd hh:mm:ss z
            ("2019-11-29 08:08-08", 1575043680000000000),
            ("2019-11-29 08:08:05-08", 1575043685000000000),
            ("2021-05-02 23:31:36.0741-07", 1620023496074100000),
            ("2019-11-29 08:15:47.624504-08", 1575044147624504000),
            ("2017-07-19 03:21:51+00:00", 1500434511000000000),
            // yyyy-mm-dd hh:mm:ss
            ("2014-04-26 05:24:37 PM", 1398533077000000000),
            ("2021-04-30 21:14", 1619817240000000000),
            ("2021-04-30 21:14:10", 1619817250000000000),
            ("2021-04-30 21:14:10.052282", 1619817250052282000),
            ("2014-04-26 17:24:37.123", 1398533077123000000),
            ("2014-04-26 17:24:37.3186369", 1398533077318636900),
            ("2012-08-03 18:31:59.257000000", 1344018719257000000),
            // yyyy-mm-dd hh:mm:ss z
            ("2014-12-16 06:20:00 UTC", 1418710800000000000),
            ("2014-04-26 13:13:43 +0800", 1398489223000000000),
            ("2014-04-26 13:13:44 +09:00", 1398485624000000000),
            ("2012-08-03 18:31:59.257000000 +0000", 1344018719257000000),
            ("2015-09-30 18:48:56.35272715 UTC", 1443638936352727150),
            // yyyy-mm-dd
            ("2021-02-21", 1613865600000000000),
            // yyyy-mon-dd
            ("2021-Feb-21", 1613865600000000000),
            // Mon dd, yyyy, hh:mm:ss
            ("May 8, 2009 5:57:51 PM", 1241805471000000000),
            ("September 17, 2012 10:09am", 1347876540000000000),
            ("September 17, 2012, 10:10:09", 1347876609000000000),
            // Mon dd, yyyy
            ("May 25, 2021", 1621900800000000000),
            ("oct 7, 1970", 24105600000000000),
            ("oct 7, 70", 24105600000000000),
            ("oct. 7, 1970", 24105600000000000),
            ("oct. 7, 70", 24105600000000000),
            ("October 7, 1970", 24105600000000000),
            // Sunday, April 18th, 2021
            ("Sunday, April 18th, 2021", 1618704000000000000),
            ("Friday, January 8th, 2021", 1610064000000000000),
            ("Tuesday, September 15th, 2020", 1600128000000000000),
            ("Monday, April 6th, 2020", 1586131200000000000),
            ("Friday, March 13th, 2020", 1584057600000000000),
            ("Thursday, May 16th, 2019", 1557964800000000000),
            // dd Mon yyyy hh:mm:ss
            ("12 Feb 2006, 19:17", 1139771820000000000),
            ("12 Feb 2006 19:17", 1139771820000000000),
            ("14 May 2019 19:11:40.164", 1557861100164000000),
            // dd Mon yyyy
            ("7 oct 70", 24105600000000000),
            ("7 oct 1970", 24105600000000000),
            ("03 February 2013", 1359849600000000000),
            ("1 July 2013", 1372636800000000000),
            // mm/dd/yyyy hh:mm:ss
            ("4/8/2014 22:05", 1396994700000000000),
            ("04/08/2014 22:05", 1396994700000000000),
            ("4/8/14 22:05", 1396994700000000000),
            ("04/2/2014 03:00:51", 1396407651000000000),
            ("8/8/1965 12:00:00 AM", -138844800000000000),
            ("8/8/1965 01:00:01 PM", -138797999000000000),
            ("8/8/1965 01:00 PM", -138798000000000000),
            ("8/8/1965 1:00 PM", -138798000000000000),
            ("8/8/1965 12:00 AM", -138844800000000000),
            ("4/02/2014 03:00:51", 1396407651000000000),
            ("03/19/2012 10:11:59", 1332151919000000000),
            ("03/19/2012 10:11:59.3186369", 1332151919318636900),
            // mm/dd/yyyy
            ("3/31/2014", 1396224000000000000),
            ("03/31/2014", 1396224000000000000),
            ("08/21/71", 51580800000000000),
            ("8/1/71", 49852800000000000),
            // yyyy/mm/dd hh:mm:ss
            ("2014/4/8 22:05", 1396994700000000000),
            ("2014/04/08 22:05", 1396994700000000000),
            ("2014/04/2 03:00:51", 1396407651000000000),
            ("2014/4/02 03:00:51", 1396407651000000000),
            ("2012/03/19 10:11:59", 1332151919000000000),
            ("2012/03/19 10:11:59.3186369", 1332151919318636900),
            // yyyy/mm/dd
            ("2014/3/31", 1396224000000000000),
            ("2014/03/31", 1396224000000000000),
            // mm.dd.yyyy
            ("3.31.2014", 1396224000000000000),
            ("03.31.2014", 1396224000000000000),
            ("08.21.71", 51580800000000000),
            // yyyy.mm.dd
            ("2014.03.30", 1396137600000000000),
            // yymmdd hh:mm:ss mysql log
            ("171113 14:14:20", 1510582460000000000),
            // chinese yyyy mm dd hh mm ss
            ("2014年04月08日11时25分18秒", 1396956318000000000),
            // chinese yyyy mm dd
            ("2014年04月08日", 1396915200000000000),
            // timezone abbreviations
            ("2017-11-25 13:31:15 PST", 1511645475000000000),
            ("2014-12-16 06:20:00 GMT", 1418710800000000000),
            ("May 26, 2021, 12:49 AM PDT", 1622015340000000000),
        ] {
            assert_eq!(*expected, parse_utc(input)?.timestamp_nanos_opt().unwrap());
        }
        Ok(())
    }
}
