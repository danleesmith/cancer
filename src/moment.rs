const DAY_S: f64 = 86400.0;

pub fn is_julian_date(year: i32, month: u32, day: u32) -> bool {
	let mut rtn = true;
	if year > 1582 { rtn = false };
	if year < 1582 { rtn = true };
	if month > 10 { rtn = false };
	if month < 10 { rtn = true };
	if day > 14 { rtn = false };
	return rtn
}

pub fn julian_day(mut year: i32, mut month: u32, day: u32, hour: u32, minute: u32, second: u32) -> f64 {
	let mut b = 0.0;

	if month <= 2 {
		year -= 1;
		month += 12;
	}

	if !is_julian_date(year, month, day) {
		let a = (year as f64 / 100.0).trunc();
		b = 2.0 - a + (a / 4.0).trunc();
	}

	let d = day as f64 + (hour as f64 / 24.0) + (minute as f64 / 1440.0) + (second as f64 / 1000.0) / DAY_S;

	let jd = (365.25 * (year + 4716) as f64).trunc() + (30.6001 * (month + 1) as f64).trunc() + d + b - 1524.5;
	
	return jd
}