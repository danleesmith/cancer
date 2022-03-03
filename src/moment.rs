const DAY_S: f64 = 86400.0;

pub fn is_julian_date(year: i32, month: u32, day: u32) -> bool {
	if year > 1582 { return false };
	if year < 1582 { return true };
	if month > 10 { return false };
	if month < 10 { return true };
	if day > 14 { return false };
	return true
}

pub fn julian_day(mut year: i32, mut month: u32, day: u32, hour: u32, minute: u32, second: u32) -> f64 {

	if month <= 2 {
		year -= 1;
		month += 12;
	}

	let mut b = 0.0;
	
	if !is_julian_date(year, month, day) {
		let a = (year as f64 / 100.0).trunc();
		b = 2.0 - a + (a / 4.0).trunc();
	}

	let t = if year < 0 { 0.75 } else { 0.0 };

	let d = day as f64 + (second + minute * 60 + hour * 3600) as f64 / DAY_S;

	let jd = b + (365.25 * year as f64 - t).trunc() + (30.6001 * (month as f64 + 1.0)).trunc() + d + 1720994.5;
	
	return jd
}