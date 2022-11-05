use std::collections::VecDeque;

use super::bsp;
use super::simple8b;
use anyhow::Context;
use anyhow::Result;
use chrono::{DateTime, FixedOffset};

pub type LatLng = (f32, f32);
type Minutes = u32;

const ORIGIN: u64 = 0;
const BIT_SHIFTS: usize = (32 - bsp::OPERATIONS) * 2 + 1;
const MAX_ERROR: f32 = 8.;

pub fn distance_meters(start: LatLng, end: LatLng) -> f32 {
    let r = 6371000.;

    let d_lat = (end.0 - start.0).to_radians();
    let d_lon = (end.1 - start.1).to_radians();
    let lat1 = (start.0).to_radians();
    let lat2 = (end.0).to_radians();

    let a = ((d_lat / 2.0).sin()) * ((d_lat / 2.0).sin())
        + ((d_lon / 2.0).sin()) * ((d_lon / 2.0).sin()) * (lat1.cos()) * (lat2.cos());
    let c = 2.0 * ((a.sqrt()).atan2((1.0 - a).sqrt()));

    r * c
}

#[derive(Default)]
pub struct GeoDB {
    beginning_minutes: u32,
    points: Vec<u64>,
}

#[derive(Default)]
pub struct Builder {
    db: GeoDB,
    last_minutes: Minutes,
    reference: u64,
    buffer: VecDeque<u64>,
    last_pos: LatLng,
}

impl Builder {
    pub fn new() -> Self {
        let db = GeoDB { ..Default::default() };
        Self {
            db,
            ..Default::default()
        }
    }

    pub fn build(mut self) -> GeoDB {
        self.flush_all();
        self.db.points.shrink_to_fit();
        self.db
    }

    fn flush_once(&mut self) {
        let (result, count) = simple8b::pack(self.buffer.as_slices().0).unwrap();
        for _ in 0..count {
            self.buffer.pop_front().unwrap();
        }
        self.db.points.push(result);
    }

    fn flush_all(&mut self) {
        while !self.buffer.is_empty() {
            self.flush_once();
        }
    }

    pub fn add(&mut self, datetime: DateTime<FixedOffset>, pos: LatLng) -> Result<bool> {
        let first_datapoint = self.db.beginning_minutes == 0;
        if first_datapoint {
            self.reference = ORIGIN;
            self.db.beginning_minutes = (datetime.timestamp() / 60) as Minutes;
        }

        let minutes = self.db.minutes_since_beginning(datetime);
        if !first_datapoint && minutes <= self.last_minutes {
            return Ok(false);
        }

        let delta_minutes = minutes - self.last_minutes;

        if !first_datapoint && delta_minutes > 1 {
            for _ in 0..delta_minutes - 1 {
                self.push(self.last_pos)?;
            }
        }

        self.last_minutes = minutes;
        self.push(pos)
    }

    fn push(&mut self, pos: LatLng) -> Result<bool> {
        let transformed = latlng_to_packed(pos);

        let delta = calculate_difference(self.reference, transformed);
        let calculated = packed_to_latlng(apply_delta(self.reference, delta));
        let error = distance_meters(pos, calculated);
        if error > MAX_ERROR {
            return Err(anyhow!("error too big: {}", error));
        }

        self.buffer.push_back(delta);
        self.reference = transformed;
        self.last_pos = pos;

        while *self.buffer.back().unwrap_or(&0) != 0 && self.buffer.len() >= 60 {
            self.flush_once();
        }

        Ok(true)
    }
}

fn calculate_difference(before: u64, after: u64) -> u64 {
    let (bigger, delta) = if after > before {
        (true, after - before)
    } else {
        (false, before - after)
    };
    let d = delta;
    let lsb = if bigger { 0 } else { u64::from(d > 0) };
    (d << 1) + lsb
}

fn latlng_to_packed(pos: LatLng) -> u64 {
    let x = (pos.0 + 90.) / 180.;
    let y = (pos.1 + 180.) / 360.;
    let n = bsp::pack2d(x, y);
    n >> BIT_SHIFTS
}

pub fn packed_to_latlng(n: u64) -> LatLng {
    let packed = n << BIT_SHIFTS;
    let (x, y) = bsp::unpack2d(packed);
    let lat = 90. * (2. * x - 1.);
    let lng = 180. * (2. * y - 1.);
    (lat, lng)
}

fn apply_delta(current: u64, unpacked: u64) -> u64 {
    let is_positive = unpacked % 2 == 0;
    let packed = unpacked >> 1;
    let packed = packed;
    if is_positive {
        current + packed
    } else {
        current - packed
    }
}

pub struct DbIter<'a> {
    db: &'a GeoDB,
    vec_id: usize,
    u64_vec: Vec<u64>,
    u64_vec_id: usize,
    current: u64,
    minutes: Minutes,
}

impl<'a> Iterator for DbIter<'a> {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.u64_vec.is_empty() {
            match self.db.points.get(self.vec_id) {
                None => return None,
                Some(&simple8b_u64) => {
                    self.u64_vec = simple8b::unpack(simple8b_u64);
                }
            };
        }

        if self.u64_vec_id >= self.u64_vec.len() {
            self.u64_vec.clear();
            self.vec_id += 1;
            self.u64_vec_id = 0;
            return self.next();
        }

        match self.u64_vec.get(self.u64_vec_id).map(|x| x.to_owned()) {
            None => None,
            Some(unpacked) => {
                self.current = apply_delta(self.current, unpacked);
                self.u64_vec_id += 1;
                self.minutes += 1;
                Some(self.current)
            }
        }
    }
}

impl<'a> IntoIterator for &'a GeoDB {
    type Item = u64;
    type IntoIter = DbIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        DbIter {
            db: self,
            vec_id: 0,
            u64_vec: vec![],
            u64_vec_id: 0,
            current: ORIGIN,
            minutes: 0,
        }
    }
}

impl GeoDB {
    fn minutes_since_beginning(&self, datetime: DateTime<FixedOffset>) -> Minutes {
        ((datetime.timestamp() / 60) as u32) - self.beginning_minutes
    }

    pub fn pos(&self, datetime: DateTime<FixedOffset>) -> Result<LatLng> {
        let target_minutes = self.minutes_since_beginning(datetime);
        let packed = self
            .into_iter()
            .take(target_minutes as usize + 1)
            .last()
            .with_context(|| {
                format!(
                    "no datapoint for {} (target_minutes = {})",
                    datetime, target_minutes
                )
            })?;
        Ok(packed_to_latlng(packed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::*;

    #[test]
    fn distances() {
        let mut db_builder = Builder::new();

        let points = vec![
            ("2022-10-21T05:16:00.444Z", -23., -46.),
            ("2022-10-21T05:17:00.444Z", -23.003, -46.006),
            ("2022-10-21T05:23:00.444Z", -23.002, -45.998),
            ("2022-10-21T05:28:00.444Z", 42., 64.),
            ("2022-10-21T05:29:00.444Z", 42.001, 63.999),
        ];

        for (time_str, lat, lng) in &points {
            let time = DateTime::parse_from_rfc3339(time_str).unwrap();
            db_builder.add(time, (*lat, *lng)).unwrap();
        }

        let db = db_builder.build();

        for (time_str, lat, lng) in points {
            let time = DateTime::parse_from_rfc3339(time_str).unwrap();
            let (latitude, longitude) = db.pos(time).unwrap();
            let error = distance_meters((lat, lng), (latitude, longitude));
            assert_le!(error, 10.);
        }

        let time = DateTime::parse_from_rfc3339("2022-10-21T05:18:00.444Z").unwrap();
        let (lat, lng) = db.pos(time).unwrap();
        assert_le!((-23.003 - lat).abs(), 0.0001);
        assert_le!((-46.006 - lng).abs(), 0.0002);

        let time = DateTime::parse_from_rfc3339("2022-10-21T05:27:00.444Z").unwrap();
        let (lat, lng) = db.pos(time).unwrap();
        assert_le!((-23.002 - lat).abs(), 0.0001);
        assert_le!((-45.998 - lng).abs(), 0.0002);
    }
}
