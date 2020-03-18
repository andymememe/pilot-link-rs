use tzdata::{
    Datetime,
    Timezone
};
use super::blob::{
    Blob,
    MAX_BLOBS
};
use super::appinfo::CategoryAppInfo;

pub const ALARM_FLAG: u8 = 64;
pub const REPEAT_FLAG: u8 = 32;
pub const NOTE_FLAG: u8 = 16;
pub const EXCEPT_FLAG: u8 = 8;
pub const DESC_FLAG: u8 = 4;
pub const LOC_FLAG: u8 = 2;

#[derive(Debug, PartialEq)]
pub enum CalendarType {
    CalendarV1,
    Unknown,
}

impl Default for CalendarType {
    fn default() -> CalendarType {
        CalendarType::CalendarV1
    }
}

pub enum CalendarRepeatType {
    CalendarRepeatNone,
    CalendarRepeatDaily,
    CalendarRepeatWeekly,
    CalendarRepeatMonthlyByDay,
    CalendarRepeatMonthlyByDate,
    CalendarRepeatYearly
}

pub enum CalendarDayOfMonthType {
    Calendar1stSun, Calendar1stMon, Calendar1stTue, Calendar1stWen, Calendar1stThu,
    Calendar1stFri,
    Calendar1stSat,
    Calendar2ndSun, Calendar2ndMon, Calendar2ndTue, Calendar2ndWen, Calendar2ndThu,
    Calendar2ndFri,
    Calendar2ndSat,
    Calendar3rdSun, Calendar3rdMon, Calendar3rdTue, Calendar3rdWen, Calendar3rdThu,
    Calendar3rdFri,
    Calendar3rdSat,
    Calendar4thSun, Calendar4thMon, Calendar4thTue, Calendar4thWen, Calendar4thThu,
    Calendar4thFri,
    Calendar4thSat,
    CalendarLastSun, CalendarLastMon, CalendarLastTue, CalendarLastWen, CalendarLastThu,
    CalendarLastFri,
    CalendarLastSat
}

pub enum CalendarAdvanceTypes {
    CalendarAdvMinutes,
    CalendarAdvHours,
    CalendarAdvDays
}

pub struct CalendarEvent<'a> {
    pub event: i32,
    pub begin: Datetime<'a>,
    pub end: Datetime<'a>,
    pub alarm: i32,
    pub advance: i32,
    pub advance_units: i32,
    pub repeat_type: CalendarRepeatType,
    pub repeat_forever: i32,
    pub repeat_end: Datetime<'a>,
    pub repeat_frequency: i32,
    pub repeat_day: CalendarDayOfMonthType,
    pub repeat_days: [i32; 7],
    pub repeat_week_start: i32,
    pub exceptions: i32,
    pub exception: Datetime<'a>,
    pub description: String,
    pub note: String,
    pub location: String,
    pub blob: [Blob; MAX_BLOBS],
    pub tz: Timezone
}

pub struct CalendarAppInfo {
    pub calendar_type: CalendarType,
    pub category: CategoryAppInfo,
    pub start_of_week: i32,
    pub internal: [u8; 18]
}

