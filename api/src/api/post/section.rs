use crate::utils::{HttpError, bad_request};

/// Map a requested section to its canonical name (`progress-logs` is the legacy alias of `logbuch`).
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
pub fn validate_season(s: &str) -> Result<(), HttpError> {
    let ok = s.split_once('/').is_some_and(|(a, b)| {
        let digits = |x: &str, n| x.len() == n && x.bytes().all(|c| c.is_ascii_digit());
        digits(a, 4) && digits(b, 2) && {
            let (a, b): (u32, u32) = (a.parse().unwrap(), b.parse().unwrap());
            b == (a + 1) % 100
        }
    });
    if ok {
        Ok(())
    } else {
        Err(bad_request("Invalid season, expected e.g. 2017/18"))
    }
}
