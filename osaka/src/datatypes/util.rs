use std::str::{
    self,
    FromStr,
};

use chrono::NaiveDateTime;

pub fn extract_value<'a>(s: &'a str) -> Option<&'a str> {
    if let Some((a, b)) = s.find("(").zip(s.find(")")) {
        return Some(&s[a + 1..b]);
    }
    None
}

pub fn extract_value_and_parse<T: FromStr>(s: &str) -> Option<T> {
    if let Ok(t) = FromStr::from_str(s) {
        Some(t)
    } else if let Some((a, b)) = s.find("(").zip(s.find(")")) {
        (&s[(a + 1)..b]).parse().ok()
    } else {
        None
    }
}

pub fn extract_datetime_string(s: &str) -> Option<&str> {
    if let Some(a) = s.find("(") {
        return Some(&s[..a]);
    }
    None
}

pub fn extract_datetime<'a>(s: &'a str) -> Option<NaiveDateTime> {
    if let Some(a) = s.find("(") {
        return NaiveDateTime::parse_from_str(&s[..a], "%Y-%m-%dT%H:%M:%S%.9f").ok();
    }
    None
}

///
/// ```rust
/// let c = $to_iter.chars().next();
/// match c {
///     Some(i) if $tag == i => (),
///     _ => return Err(()),
/// };
/// ```
#[macro_export]
macro_rules! tag_guard {
    ($tag:literal, $to_iter:ident) => {
        let c = $to_iter.chars().next();
        match c {
            Some(i) if $tag == i => (),
            _ => return Err(()),
        };
    };
}
