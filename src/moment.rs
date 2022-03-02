pub struct Moment {
	year: i32,
	month: i32, 
	day: i32, 
	hour: i32, 
	minute: i32, 
	second: i32, 
	millisecond: i32 ,
}

impl Moment {
	pub fn new (y: i32, m: i32, d: i32, hr: i32, mn: i32, sc: i32, ms: i32) -> Moment {
		Moment {
			year: y,
			month: m, 
			day: d, 
			hour: hr, 
			minute: mn, 
			second: sc, 
			millisecond: ms,
		}
	}

	pub fn day_of_month(&self) -> f64 {
		self.day as f64 + 
		(self.hour as f64 / 24.0) + 
		(self.minute as f64 / 1440.0) + 
		(self.second as f64 + self.millisecond as f64 / 1000.0) / 86400.0
	}

	pub fn is_julian_date(&self) -> bool {
		let mut rtn = true;
		if self.year > 1582 { rtn = false };
		if self.year < 1582 { rtn = true };
		if self.month > 10 { rtn = false };
		if self.month < 10 { rtn = true };
		if self.day > 14 { rtn = false };
		rtn
	}

	pub fn julian_day(&self) -> f64 {
		let mut y = self.year;
		let mut m = self.month;
		let mut b = 0.0;

		if m == 1 || m == 2 {
			y -= 1;
			m += 12;
		}

		if !self.is_julian_date() {
			let a = (y as f64 / 100.0).trunc();
			b = 2.0 - a + (a / 4.0).trunc();
		}

		let jd = (365.25 * (y + 4716) as f64).trunc() + (30.6001 * (m + 1) as f64).trunc() + self.day_of_month() + b - 1524.5;
		
		jd
	}
}