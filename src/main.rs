mod moment;
use crate::moment::*;

use::std::collections::HashMap;

fn main() {

	let jd_test = HashMap::from([
		("Sputnik Launch", julian_day(1957, 10, 4, 19, 29, 0)),
		("2000 Jan. 1.5", julian_day(2000, 1, 1, 12, 0, 0)),
		("1999 Jan. 1.0", julian_day(1999, 1, 1, 0, 0, 0)),
		("1987 Jan. 27.0", julian_day(1987, 1, 27, 0, 0, 0)),
		("1987 Jun. 19.5", julian_day(1987, 6, 19, 12, 0, 0)),
		("1988 Jan. 27.0", julian_day(1988, 1, 27, 0, 0, 0)),
		("1900 Jan. 1.0", julian_day(1900, 1, 1, 0, 0, 0)),
		("1600 Jan. 1.0", julian_day(1600, 1, 1, 0, 0, 0)),
		("1600 Dec. 31.0", julian_day(1600, 12, 31, 0, 0, 0)),
		("837 Apr. 10.3", julian_day(837, 4, 10, 7, 12, 0)),
		("-123 Dec. 31.0", julian_day(-123, 12, 31, 0, 0, 0)),
		("-122 Jan. 1.0", julian_day(-122, 1, 1, 0, 0, 0)),
		("-1000 July. 12.5", julian_day(-1000, 7, 12, 12, 0, 0)),
		("-1000 Feb. 29.0", julian_day(-1000, 2, 29, 0, 0, 0)),
		("-1001 Aug. 17.9", julian_day(-1001, 9, 17, 21, 36, 0)),
		("-4712 Jan. 1.5", julian_day(-4712, 1, 1, 12, 0, 0)),
	]);

	for (date, jd) in &jd_test {
		println!("Date: {} | Julian Date: {}", date, jd);
	}
}