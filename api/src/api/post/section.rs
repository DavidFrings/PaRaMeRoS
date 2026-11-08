use crate::utils::{HttpError, bad_request};
use chrono::{DateTime, Datelike, NaiveDateTime, TimeDelta, Utc};

/// Map a requested section to its canonical name (`progress-logs` is the legacy alias of `logbuch`).
///
/// # Errors
/// 400 for an unknown section.
pub fn normalize_section(raw: &str) -> Result<&'static str, HttpError> {
    match raw {
        "logbuch" | "progress-logs" => Ok("logbuch"),
        "team" => Ok("team"),
        "coaches" => Ok("coaches"),
        "forest-project" => Ok("forest-project"),
        _ => Err(bad_request("Unknown section")),
    }
}

/// Season format `YYYY/YY`, where the second part is the first year + 1 (mod 100).
///
/// # Errors
/// 400 if `s` is not a valid season.
pub fn validate_season(s: &str) -> Result<(), HttpError> {
    let ok = s.split_once('/').is_some_and(|(a, b)| {
        let digits = |x: &str, n| x.len() == n && x.bytes().all(|c| c.is_ascii_digit());
        digits(a, 4)
            && digits(b, 2)
            && matches!((a.parse::<u32>(), b.parse::<u32>()), (Ok(a), Ok(b)) if b == (a + 1) % 100)
    });
    if ok {
        Ok(())
    } else {
        Err(bad_request("Invalid season, expected e.g. 2017/18"))
    }
}

/// RFC 3339 with an offset to the naive UTC value `posts.created_at` stores. Allowed range: from
/// 2017 to one day ahead.
///
/// # Errors
/// 400 if `raw` is not RFC 3339 or out of range.
pub fn parse_created_at(raw: &str) -> Result<NaiveDateTime, HttpError> {
    let invalid = || bad_request("Invalid created_at");
    let t = DateTime::parse_from_rfc3339(raw)
        .map_err(|_| invalid())?
        .naive_utc();
    if t.year() < 2017 || t > Utc::now().naive_utc() + TimeDelta::days(1) {
        return Err(invalid());
    }
    Ok(t)
}
