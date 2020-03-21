use super::appinfo::CategoryAppInfo;
use super::blob::{unpack_blob, Blob, BLOB_TYPE_CALENDAR_TIMEZONE_ID, MAX_BLOBS};
use super::{get_buf_string, get_short};

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

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum CalendarRepeatType {
	CalendarRepeatNone,
	CalendarRepeatDaily,
	CalendarRepeatWeekly,
	CalendarRepeatMonthlyByDay,
	CalendarRepeatMonthlyByDate,
	CalendarRepeatYearly,
}

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum CalendarDayOfMonthType {
	Calendar1stSun,
	Calendar1stMon,
	Calendar1stTue,
	Calendar1stWen,
	Calendar1stThu,
	Calendar1stFri,
	Calendar1stSat,
	Calendar2ndSun,
	Calendar2ndMon,
	Calendar2ndTue,
	Calendar2ndWen,
	Calendar2ndThu,
	Calendar2ndFri,
	Calendar2ndSat,
	Calendar3rdSun,
	Calendar3rdMon,
	Calendar3rdTue,
	Calendar3rdWen,
	Calendar3rdThu,
	Calendar3rdFri,
	Calendar3rdSat,
	Calendar4thSun,
	Calendar4thMon,
	Calendar4thTue,
	Calendar4thWen,
	Calendar4thThu,
	Calendar4thFri,
	Calendar4thSat,
	CalendarLastSun,
	CalendarLastMon,
	CalendarLastTue,
	CalendarLastWen,
	CalendarLastThu,
	CalendarLastFri,
	CalendarLastSat,
}

pub enum CalendarAdvanceTypes {
	CalendarAdvMinutes,
	CalendarAdvHours,
	CalendarAdvDays,
}

#[derive(Debug, PartialEq)]
pub struct CalendarEvent {
	pub event: i32,
	// pub begin: Datetime<'a>,
	// pub end: Datetime<'a>,
	pub alarm: i32,
	pub advance: u8,
	pub advance_units: u8,
	pub repeat_type: CalendarRepeatType,
	pub repeat_forever: i32,
	// pub repeat_end: Datetime<'a>,
	pub repeat_frequency: u8,
	pub repeat_day: CalendarDayOfMonthType,
	pub repeat_days: [u8; 7],
	pub repeat_week_start: u8,
	pub exceptions: u16,
	// pub exception: Datetime<'a>,
	pub description: String,
	pub note: String,
	pub location: String,
	pub blob: [Blob; MAX_BLOBS],
	// pub tz: Timezone
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CalendarAppInfo {
	// pub calendar_type: CalendarType,
	// pub category: CategoryAppInfo,
	pub start_of_week: i32,
	pub internal: [u8; 18],
}

pub fn unpack_calendar_event(a: &mut CalendarEvent, buf: &Vec<u8>, cal_type: CalendarType) -> i32 {
	let iflags: u8;
	let mut result: usize;
	let mut d: u16;
	let mut p2_offset: usize;
	let destlen: usize = 8;

	match cal_type {
		CalendarType::CalendarV1 => {}
		CalendarType::Unknown => return -1,
	}
	if buf.len() < destlen {
		return -1;
	}

	d = get_short(&buf[4..6]);
	// a.begin = NaiveDateTime::from_ymd((d >> 9) + 4, ((d >> 5) & 15) - 1, d & 31).and_hms(buf[0], buf[1], 0);
	// a.end = NaiveDateTime::from_ymd((d >> 9) + 4, ((d >> 5) & 15) - 1, d & 31).and_hms(buf[2], buf[3], 0);

	if get_short(&buf[0..2]) == 0xffff {
		a.event = 1;
	// a.begin.tm_hour = 0;
	// a.begin.tm_min = 0;
	// a.end.tm_hour = 0;
	// a.end.tm_min = 0;
	} else {
		a.event = 0;
	}

	iflags = buf[6];
	p2_offset = 8;

	if (iflags & ALARM_FLAG) != 0 {
		a.alarm = 1;
		a.advance = buf[p2_offset];
		p2_offset += 1;
		a.advance_units = buf[p2_offset];
	} else {
		a.alarm = 0;
		a.advance = 0;
		a.advance_units = 0;
	}

	if (iflags & REPEAT_FLAG) != 0 {
		let on: u8;

		a.repeat_type = num::FromPrimitive::from_u8(buf[p2_offset]).unwrap();
		d = get_short(&buf[p2_offset..p2_offset + 2]);
		p2_offset += 2;
		if d == 0xffff {
			a.repeat_forever = 1; /* repeatEnd is invalid */
		} else {
			// a.repeat_end.tm_year 	= (d >> 9) + 4;
			// a.repeat_end.tm_mon 	= ((d >> 5) & 15) - 1;
			// a.repeat_end.tm_mday 	= d & 31;
			// a.repeat_end.tm_min 	= 0;
			// a.repeat_end.tm_hour 	= 0;
			// a.repeat_end.tm_sec 	= 0;
			// a.repeat_end.tm_isdst 	= -1;
			a.repeat_forever = 0;
		}
		a.repeat_frequency = buf[p2_offset];
		p2_offset += 1;
		on = buf[p2_offset];
		p2_offset += 1;

		a.repeat_day = num::FromPrimitive::from_u32(0).unwrap();

		for i in 0..7 {
			a.repeat_days[i] = 0;
		}

		if a.repeat_type == CalendarRepeatType::CalendarRepeatMonthlyByDay {
			a.repeat_day = num::FromPrimitive::from_u8(on).unwrap();
		} else if a.repeat_type == CalendarRepeatType::CalendarRepeatWeekly {
			for i in 0..7 {
				a.repeat_days[i] = !!(on & (1 << i));
			}
		}
		a.repeat_week_start = buf[p2_offset];
		p2_offset += 2;
	} else {
		a.repeat_type = num::FromPrimitive::from_u8(0).unwrap();
		a.repeat_forever = 1; /* repeatEnd is invalid */
		a.repeat_frequency = 0;
		a.repeat_day = num::FromPrimitive::from_u32(0).unwrap();
		for i in 0..7 {
			a.repeat_days[i] = 0;
		}
		a.repeat_week_start = 0;
	}

	if (iflags & EXCEPT_FLAG) != 0 {
		a.exceptions = get_short(&buf[p2_offset..p2_offset + 2]);
		p2_offset += 2;
		// a.exception = NaiveDateTime::default();

		for j in 0..a.exceptions {
			d = get_short(&buf[p2_offset..p2_offset + 2]);
			// a.exception[j].tm_year = (d >> 9) + 4;
			// a.exception[j].tm_mon = ((d >> 5) & 15) - 1;
			// a.exception[j].tm_mday = d & 31;
			// a.exception[j].tm_hour = 0;
			// a.exception[j].tm_min = 0;
			// a.exception[j].tm_sec = 0;
			// a.exception[j].tm_isdst = -1;
			p2_offset += 2;
		}
	} else {
		a.exceptions = 0;
		// a.exception 	= 0;
	}

	if (iflags & DESC_FLAG) != 0 {
		let (_desc, _offset) = get_buf_string(&buf, p2_offset);
		a.description = _desc;
		p2_offset = _offset + 1;
	} else {
		a.description = String::from("");
	}

	if (iflags & NOTE_FLAG) != 0 {
		let (_note, _offset) = get_buf_string(&buf, p2_offset);
		a.note = _note;
		p2_offset = _offset + 1;
	} else {
		a.note = String::from("");
	}

	if (iflags & LOC_FLAG) != 0 {
		let (_loc, _offset) = get_buf_string(&buf, p2_offset);
		a.location = _loc;
		p2_offset = _offset + 1;
	} else {
		a.location = String::from("");
	}

	/* initialize the blobs to NULL */
	for i in 0..MAX_BLOBS {
		a.blob[i] = Blob::default();
	}

	if p2_offset < buf.len() {
		let mut blob_count: usize = 0;
		/* read the blobs */
		// a.tz = NULL;
		while (buf.len() - p2_offset) > 6 {
			if blob_count >= MAX_BLOBS {
				/* too many blobs were found */
				println!(
					"Error, found more than {} blobs: {}\n",
					MAX_BLOBS, blob_count
				);
				return -1;
			}

			result = unpack_blob(&mut a.blob[blob_count], &buf[p2_offset..].to_vec(), 0);
			p2_offset += result;
			/* if it's a timezone blob store it */
			if a.blob[blob_count].blob_type == BLOB_TYPE_CALENDAR_TIMEZONE_ID {
				// if NULL != a.tz {
				// 	println!("Warning: Found more than one timezone blob! Freeing the previous one and starting again\n");
				// }
				// result = unpack_Timezone_p(a.tz, a.blob[blob_count].data, 0);
				if result != a.blob[blob_count].length {
					println!(
						"Read the wrong number of bytes for a timezone expected {} but was {}\n",
						a.blob[blob_count].length, result
					);
					return -1;
				}
			}
			blob_count += 1;
		}

		if p2_offset < buf.len() {
			println!("Extra data found {} bytes\n", (buf.len() - p2_offset));
			return -1;
		}
	} else {
		// a.tz = NULL;
	}
	return 0;
}
