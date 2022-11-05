use super::db::Builder as DbBuilder;
use super::db::GeoDB as Db;
use crate::fs;
use anyhow::{Context, Result};
use chrono::{DateTime, Duration, FixedOffset};
use std::ops::Add;
use std::ops::Sub;
use std::path::Path;

/*
This function assumes input of the following format:
2022-10-21T05:16:00.444Z 594188745 -235856546 -466419103
2022-10-21T05:21:39.198Z 594188745 -235856546 -466419103

To convert the raw location history JSON to this format, run:
❯ cat "Records.json" | grep -e '^    "latitudeE7"' -e '^    "longitudeE7"' -e '^    "timestamp' -e '^    "deviceTag' | tr -d '",' | awk '{print $2}' | sed 'N;N;N;s/\n/ /g' | awk '{print $4, $3, $1, $2}' > "Records_simple.txt"
*/
pub fn get_db(path: &Path, min_time: DateTime<FixedOffset>, max_time: DateTime<FixedOffset>) -> Result<Db> {
    let duration = Duration::days(1);
    let format = "%Y-%m-%dT";
    let find_begin = min_time.sub(duration).format(format).to_string();
    let find_end = max_time.add(duration).format(format).to_string();

    let mut builder = DbBuilder::new();

    let lines = fs::read_lines(path)?;
    let mut ignore = true;
    let mut points = 0;
    let mut last_data = None;

    for line in lines {
        if ignore && line.starts_with(&find_begin) {
            ignore = false
        } else if !ignore && line.starts_with(&find_end) {
            break;
        } else if ignore {
            continue;
        }

        let mut parts = line.split(' ');

        let date_str = parts.next().context("no timestamp")?;
        let device_str = parts.next().context("no device")?;
        let lat_str = parts.next().context("no lat")?;
        let lng_str = parts.next().context("no lng")?;

        let device = device_str.parse::<i32>()?;
        if device != -363621992 {
            continue;
        }

        let lat = (lat_str.parse::<i32>()? as f32) / 10000000.0;
        if !(-90.0..=90.0).contains(&lat) {
            return Err(anyhow!("invalid lat: {lat}"));
        }

        let lng = (lng_str.parse::<i32>()? as f32) / 10000000.0;
        if !(-180.0..=180.0).contains(&lng) {
            return Err(anyhow!("invalid lng: {lng}"));
        }

        let date = DateTime::parse_from_rfc3339(date_str).context("invalid date")?;
        builder.add(date, (lat, lng))?;
        last_data = Some((date, (lat, lng)));
        points += 1;
    }

    let db = builder.build();
    println!("input points: {} k", (points / 1000) as u32);
    dbg!(last_data.unwrap());

    Ok(db)
}
