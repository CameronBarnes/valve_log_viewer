use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::types::{Entry, Log, SharedLog};

static CACHE: Mutex<Vec<Arc<str>>> = Mutex::new(Vec::new());

/// If 'input' is present in the cache then return a copy of that Arc, otherwise create and add a
/// new one to the cache
fn get_level(input: &str) -> Arc<str> {
    let mut cache = CACHE.lock().unwrap();
    if let Some(out) = cache.iter().find(|item| input.eq_ignore_ascii_case(item)) {
        out.clone()
    } else {
        let out: Arc<str> = Arc::from(input);
        cache.push(out.clone());
        out
    }
}

/// Return a Vec of all registered logging levels
pub fn get_levels() -> Vec<Arc<str>> {
    CACHE.lock().unwrap().clone()
}

/// Parse a log entry into an `Entry` object with timestamp, level, and data
fn parse(log_line: [&str; 3]) -> Result<Entry> {
    static TIME_RE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r#"(\d{2}):(\d{2}):(\d{2}):(\d+)"#).unwrap());
    let (timestamp, level, data) = log_line.into();
    // Split the timestamp into date and time separately
    let (date, time) = timestamp
        .split_once(' ') // We dont need the day of the week so we're discarding it here
        .ok_or(anyhow!("Failed to split timestamp"))?
        .1
        .split_at(11); // This is where we split the time and date
                       // Use regex to parse out the components from the time portion of the timestamp
    let (hour, min, sec, micro) = TIME_RE
        .captures(time)
        .ok_or(anyhow!("Failed to parse time"))?
        .extract()
        .1
        .into();
    // I dont mind unwrapping these because the regex already confirmed that they're digits of
    // acceptable length
    let (hour, min, sec, micro) = (
        hour.parse().unwrap(),
        min.parse().unwrap(),
        sec.parse().unwrap(),
        micro.parse().unwrap(),
    );
    // First we let the parse_from_str function parse the date from the date string, then we add on
    // the time component from the parts we got with the regex
    let timestamp = NaiveDate::parse_from_str(date, "%b %d %Y")?
        .and_hms_micro_opt(hour, min, sec, micro)
        .ok_or(anyhow!("Failed to build DateTime object"))?;

    let level = get_level(level);

    Ok(Entry::new(timestamp, level, data))
}

// Parses an input line and adds it to the `SharedLog`. Creates a new `Entry` if required or
// appends the data to the previous entry if appropreate.
pub fn parse_line(log: &mut Log, line: &str) -> Result<()> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r#"(\w{3} \w{3} \d{2} \d{4} \d{2}:\d{2}:\d{2}.\d+) \[(\w+?)\] - (.+?)$"#)
            .unwrap()
    });
    // If it's a match we pass it through to the parse function, otherwise we append to the
    // previous log entry
    if let Some(captures) = RE.captures(line) {
        let entry = parse(captures.extract().1)?;
        log.add_entry(entry);
        Ok(())
    } else {
        log.append_last(line)
    }
}

pub fn parse_file_path<T: Into<PathBuf>>(
    path: T,
    extension: &str,
) -> Result<Vec<(SharedLog, PathBuf)>> {
    let path: PathBuf = path.into();

    let _ = path.try_exists()?;

    if path.is_file()
        && path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case(extension))
    {
        Ok(vec![(
            Arc::new(Mutex::new(Log::new(
                path.file_name().unwrap().to_string_lossy(),
            ))),
            path,
        )])
    } else {
        Ok(path
            .read_dir()?
            .flatten()
            .flat_map(|entry| parse_file_path(entry.path(), extension))
            .flatten()
            .collect())
    }
}
