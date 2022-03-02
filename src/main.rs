fn julian_day(mut year: f64, mut month: f64, day: f64, hour: f64, minute: f64, second: f64, millisecond: f64) -> f64 {

	let doty = day + (hour / 24.0) + (minute / 1440.0) + (second + millisecond / 1000.0) / 86400.0;
	let mut b = 0.0;

	if month == 1.0 || month == 2.0 {
		year -= 1.0;
		month += 12.0;
	}

	if !is_julian_date(year, month, day) {
		let a = (year / 100.0).floor();
		b = 2.0 - (a / 4.0).floor();
	}

	(365.25 * (year + 4716.0)).floor() + (30.6001 * (month + 1.0)).floor() + doty + b - 1524.5

}

fn is_julian_date(year: f64, month: f64, day: f64) -> bool {
	let mut cond = true;
	if year > 1582.0 { cond = false };
	if year < 1582.0 { cond = true };
	if month > 10.0 { cond = false };
	if month < 10.0 { cond = true };
	if day > 14.0 { cond = false };
	cond
}

fn main() {
    println!("JD: {}", julian_day(1957.0, 10.0, 4.0, 19.0, 29.0, 0.0, 0.0));
}