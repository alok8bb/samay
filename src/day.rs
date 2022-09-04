use chrono::{DateTime, Local, Timelike};

pub fn calc_day(time: &DateTime<Local>) -> f32 {
    let passed_seconds = time.hour() * 3600 + time.minute() * 60;
    let total_seconds = 24 * 3600;

    ((passed_seconds as f32) / (total_seconds as f32)) * 100f32
}
