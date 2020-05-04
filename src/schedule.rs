use chrono::{
	DateTime, Datelike, Duration, FixedOffset, NaiveTime, TimeZone, Timelike, Utc, Weekday,
};

pub struct Schedule {
	weekday: Weekday,
	time: NaiveTime,
	offset: FixedOffset,
	// TODO: Display it
	#[allow(dead_code)]
	unverified_time: bool,
}

impl Schedule {
	pub fn new(weekday: Weekday, time: Option<NaiveTime>, offset_hours: i32) -> Schedule {
		Schedule {
			weekday,
			time: time.unwrap_or_else(|| NaiveTime::from_hms(18, 00, 00)),
			offset: FixedOffset::east(offset_hours * 3600),
			unverified_time: time.is_none(),
		}
	}

	// TODO: Use that when showing count down
	#[allow(dead_code)]
	pub fn time_to_next(&self) -> Duration {
		self.time_to_next_from_time(Utc::now())
	}

	pub fn next(&self) -> DateTime<Utc> {
		let now = Utc::now();
		self.next_from_time(now)
	}

	pub fn time_to_next_from_time(&self, time: DateTime<Utc>) -> Duration {
		let next = self.next_from_time(time);
		next - time
	}

	pub fn unverified(&self) -> bool {
		self.unverified_time
	}

	/// Calculate when the next session should be based
	/// on the current time.
	pub fn next_from_time(&self, time: DateTime<Utc>) -> DateTime<Utc> {
		let time = time.with_timezone(&self.offset);
		let weekday = self.weekday.num_days_from_monday();
		let current_weekday = time.weekday().num_days_from_monday();

		let days_to = if current_weekday <= weekday {
			weekday - current_weekday
		} else {
			7 + weekday - current_weekday
		};

		let mut next = self
			.offset
			.ymd(time.year(), time.month(), time.day())
			.and_hms(self.time.hour(), self.time.minute(), self.time.second());
		next = next + Duration::days(days_to as i64);
		if next < time {
			// Session was today
			next = next + Duration::days(7)
		}
		next.with_timezone(&Utc)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::Local;

	#[test]
	fn next_session_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, Some(NaiveTime::from_hms(10, 0, 0)), 0);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, Some(NaiveTime::from_hms(10, 0, 0)), 0);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, Some(NaiveTime::from_hms(9, 0, 0)), 0);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 9).and_hms(9, 00, 00)
		);
	}

	#[test]
	fn one_day_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, Some(NaiveTime::from_hms(10, 0, 0)), 0);

		assert_eq!(
			schedule.time_to_next_from_time(current_time),
			Duration::days(1)
		);
	}

	#[test]
	fn one_hour_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, Some(NaiveTime::from_hms(11, 0, 0)), 0);

		assert_eq!(
			schedule.time_to_next_from_time(current_time),
			Duration::hours(1)
		);
	}

	#[test]
	fn three_days_to_next_session_next_week() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Tue, Some(NaiveTime::from_hms(10, 0, 0)), 0);

		assert_eq!(
			schedule.time_to_next_from_time(current_time),
			Duration::days(3)
		);
	}

	#[test]
	fn six_days_twenty_three_hours_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, Some(NaiveTime::from_hms(9, 0, 0)), 0);

		assert_eq!(
			schedule.time_to_next_from_time(current_time),
			Duration::days(6) + Duration::hours(23)
		);
	}

	#[test]
	fn next_session_in_east_timezone_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, Some(NaiveTime::from_hms(12, 0, 0)), 2);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, Some(NaiveTime::from_hms(8, 0, 0)), -2);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_east_timezone_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, Some(NaiveTime::from_hms(12, 0, 0)), 2);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, Some(NaiveTime::from_hms(8, 0, 0)), -2);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_east_timezone_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(20, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, Some(NaiveTime::from_hms(5, 0, 0)), 10);
		println!(
			"{}",
			Utc.ymd(2020, 5, 1).and_hms(12, 0, 0).with_timezone(&Local)
		);
		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 9).and_hms(19, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(20, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, Some(NaiveTime::from_hms(9, 0, 0)), -10);

		assert_eq!(
			schedule.next_from_time(current_time),
			Utc.ymd(2020, 5, 9).and_hms(19, 00, 00)
		);
	}
}
