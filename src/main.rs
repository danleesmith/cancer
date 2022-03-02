mod moment;
use crate::moment::Moment;

fn main() {
	let moment = Moment::new(1957, 10, 4, 19, 29, 0, 0);
    println!("JD: {}", moment.julian_day());
	println!("JD?: {}", moment.is_julian_date());
	println!("DoM: {}", moment.day_of_month());

}