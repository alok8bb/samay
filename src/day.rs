use chrono::{DateTime, Datelike, Local, Timelike};

pub enum FirstDay {
    Monday,
    Sunday,
}

const SECONDS_IN_DAY: u32 = 24 * 3600;
const DAYS_IN_WEEK: f32 = 7f32;
const DAYS_IN_YEAR: f32 = 366f32;

pub fn calc_day(time: &DateTime<Local>) -> f32 {
    let passed_seconds = time.num_seconds_from_midnight() as f32;
    let total_seconds = SECONDS_IN_DAY as f32;

    (passed_seconds / total_seconds) * 100f32
}

pub fn calc_week(time: &DateTime<Local>, first_day: FirstDay, precise: bool) -> f32 {
    if precise {
        // TODO: calculate from midnight of first week day
    }

    let days_from_first_day = match first_day {
        FirstDay::Monday => time.weekday().num_days_from_monday(),
        FirstDay::Sunday => time.weekday().num_days_from_sunday(),
    } as f32;

    (days_from_first_day / DAYS_IN_WEEK) * 100f32
}

pub fn calc_year(time: &DateTime<Local>) -> f32 {
    (time.ordinal() as f32 / DAYS_IN_YEAR) * 100f32
}
