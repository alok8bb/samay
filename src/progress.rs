use std::ops::Add;

use chrono::{DateTime, Datelike, Local, NaiveDate, Timelike};

pub enum FirstDay {
    Monday,
    Sunday,
}

const SECONDS_IN_DAY: u32 = 24 * 3600;
const SECONDS_IN_WEEK: u32 = DAYS_IN_WEEK * SECONDS_IN_DAY;
const DAYS_IN_WEEK: u32 = 7;
const DAYS_IN_YEAR: u32 = 366; // using leap year days (progress gets rounded)

pub fn calc_day(time: &DateTime<Local>) -> f32 {
    let passed_seconds = time.num_seconds_from_midnight() as f32;
    let total_seconds = SECONDS_IN_DAY as f32;

    (passed_seconds / total_seconds) * 100f32
}

pub fn calc_week(time: &DateTime<Local>, first_day: FirstDay, precise: bool) -> f32 {
    let days_from_first_day = match first_day {
        FirstDay::Monday => time.weekday().num_days_from_monday(),
        FirstDay::Sunday => time.weekday().num_days_from_sunday(),
    };

    if precise {
        let secs_since_first_weekday =
            (days_from_first_day * SECONDS_IN_DAY).add(time.num_seconds_from_midnight()) as f32;
        return (secs_since_first_weekday / SECONDS_IN_WEEK as f32) * 100f32;
    }

    (days_from_first_day as f32 / DAYS_IN_WEEK as f32) * 100f32
}

pub fn calc_month(time: &DateTime<Local>) -> f32 {
    (time.day() as f32 / (get_days_from_month(time.year(), time.month()) as f32)) * 100f32
}

pub fn calc_year(time: &DateTime<Local>) -> f32 {
    (time.ordinal() as f32 / DAYS_IN_YEAR as f32) * 100f32
}

fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}
