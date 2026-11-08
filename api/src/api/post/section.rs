use crate::utils::{HttpError, bad_request};

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
