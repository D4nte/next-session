use chrono::{
	DateTime, Datelike, Duration, FixedOffset, Local, NaiveTime, TimeZone, Timelike, Utc, Weekday,
};
use yew::{prelude::*, Component, ComponentLink, Html, ShouldRender};

impl Component for Session {
	type Message = ();
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		unimplemented!()
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		// Nothing to do here
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		web_sys::console::log_1(&"view on session".into());
		let club = self.club.clone();
		let time = utc_to_local(&self.schedule.next()).to_string();
		html! {
		<div class="card bg-light mb-3" style="">
			<div class="card-body">
				<h5 class="card-title">{ club }</h5>
				<p class="card-text">{ time }</p>
			</div>
		</div>
		}
	}
}

pub struct Session {
	pub club: String,
	pub schedule: Schedule,
}

pub struct Schedule {
	weekday: Weekday,
	time: NaiveTime,
	offset: FixedOffset,
}

impl Schedule {
	pub fn new(weekday: Weekday, time: NaiveTime, offset_hours: i32) -> Schedule {
		Schedule {
			weekday,
			time,
			offset: FixedOffset::east(offset_hours * 3600),
		}
	}

	// Will use that when showing count down
	#[allow(dead_code)]
	pub fn time_to_next(&self) -> Duration {
		self.time_to_next_from_current(Utc::now())
	}

	pub fn next(&self) -> DateTime<Utc> {
		let now = Utc::now();
		self.next_from_current(now)
	}

	pub fn time_to_next_from_current(&self, time: DateTime<Utc>) -> Duration {
		let next = self.next_from_current(time);
		next - time
	}

	/// Calculate when the next session should be based
	/// on the current time.
	pub fn next_from_current(&self, current_time: DateTime<Utc>) -> DateTime<Utc> {
		let current_time = current_time.with_timezone(&self.offset);
		let weekday = self.weekday.num_days_from_monday();
		let current_weekday = current_time.weekday().num_days_from_monday();

		let days_to = if current_weekday <= weekday {
			weekday - current_weekday
		} else {
			7 + weekday - current_weekday
		};

		let mut next = self
			.offset
			.ymd(
				current_time.year(),
				current_time.month(),
				current_time.day(),
			)
			.and_hms(self.time.hour(), self.time.minute(), self.time.second());
		next = next + Duration::days(days_to as i64);
		if next < current_time {
			// Session was today
			next = next + Duration::days(7)
		}
		next.with_timezone(&Utc)
	}
}

// Until https://github.com/chronotope/chrono/pull/412/ gets merged and released
fn utc_to_local(utc: &DateTime<Utc>) -> DateTime<Local> {
	let utc = utc.naive_utc();
	let offset = FixedOffset::west((js_sys::Date::new_0().get_timezone_offset() as i32) * 60);
	DateTime::from_utc(utc, offset)
}

#[cfg(test)]
impl Default for Session {
	fn default() -> Self {
		Session {
			club: "UTS Jitsu".to_string(),
			schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(19, 15, 00), 10),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn next_session_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(10, 0, 0), 0);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, NaiveTime::from_hms(10, 0, 0), 0);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(9, 0, 0), 0);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 9).and_hms(9, 00, 00)
		);
	}

	#[test]
	fn one_day_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(10, 0, 0), 0);

		assert_eq!(
			schedule.time_to_next_from_current(current_time),
			Duration::days(1)
		);
	}

	#[test]
	fn one_hour_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(11, 0, 0), 0);

		assert_eq!(
			schedule.time_to_next_from_current(current_time),
			Duration::hours(1)
		);
	}

	#[test]
	fn three_days_to_next_session_next_week() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Tue, NaiveTime::from_hms(10, 0, 0), 0);

		assert_eq!(
			schedule.time_to_next_from_current(current_time),
			Duration::days(3)
		);
	}

	#[test]
	fn six_days_twenty_three_hours_to_next_session() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(09, 0, 0), 0);

		assert_eq!(
			schedule.time_to_next_from_current(current_time),
			Duration::days(6) + Duration::hours(23)
		);
	}

	#[test]
	fn next_session_in_east_timezone_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(12, 0, 0), 2);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_tomorrow_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(8, 0, 0), -2);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 3).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_east_timezone_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, NaiveTime::from_hms(12, 0, 0), 2);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_in_six_days_same_time() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(10, 00, 00);

		let schedule = Schedule::new(Weekday::Fri, NaiveTime::from_hms(8, 0, 0), -2);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 8).and_hms(10, 00, 00)
		);
	}

	#[test]
	fn next_session_in_east_timezone_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(20, 00, 00);

		let schedule = Schedule::new(Weekday::Sun, NaiveTime::from_hms(5, 0, 0), 10);
		println!(
			"{}",
			Utc.ymd(2020, 05, 01)
				.and_hms(12, 0, 0)
				.with_timezone(&Local)
		);
		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 9).and_hms(19, 00, 00)
		);
	}

	#[test]
	fn next_session_in_west_timezone_in_six_days_twenty_three_hours() {
		// Saturday 2nd of May
		let current_time = Utc.ymd(2020, 5, 2).and_hms(20, 00, 00);

		let schedule = Schedule::new(Weekday::Sat, NaiveTime::from_hms(9, 0, 0), -10);

		assert_eq!(
			schedule.next_from_current(current_time),
			Utc.ymd(2020, 5, 9).and_hms(19, 00, 00)
		);
	}
}
