use crate::schedule::Schedule;
use chrono::{DateTime, FixedOffset, Local, Utc};
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
		let time = utc_to_local(&self.schedule.next()).format("%A %H:%M");
		html! {
		<div class="card bg-light mt-3 mb-3" style="">
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

// Until https://github.com/chronotope/chrono/pull/412/ gets merged and released
fn utc_to_local(utc: &DateTime<Utc>) -> DateTime<Local> {
	let utc = utc.naive_utc();
	#[allow(clippy::cast_possible_truncation)]
	// Timezone offset cannot be bigger than i32::max
	let offset = FixedOffset::west((js_sys::Date::new_0().get_timezone_offset() as i32) * 60);
	DateTime::from_utc(utc, offset)
}

#[cfg(test)]
impl Default for Session {
	fn default() -> Self {
		use chrono::{NaiveTime, Weekday};
		Session {
			club: "UTS Jitsu".to_string(),
			schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(19, 15, 00), 10),
		}
	}
}

impl Session {
	pub fn order(mut sessions: Vec<Session>, time: DateTime<Utc>) -> Vec<Session> {
		sessions.sort_by(|a, b| {
			a.schedule
				.time_to_next_from_current(time)
				.cmp(&b.schedule.time_to_next_from_current(time))
		});
		sessions
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{NaiveTime, TimeZone, Weekday};

	#[test]
	fn schedules_are_ordered_chronologically() {
		let sessions = vec![
			Session {
				club: "Second".to_string(),
				schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(19, 00, 00), 0),
			},
			Session {
				club: "First".to_string(),
				schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(20, 00, 00), 3),
			},
			Session {
				club: "Third".to_string(),
				schedule: Schedule::new(Weekday::Mon, NaiveTime::from_hms(10, 00, 00), 1),
			},
		];

		let ordered = Session::order(
			sessions,
			Utc.isoywd(2020, 2, Weekday::Tue).and_hms(12, 00, 00),
		);

		assert_eq!(ordered.get(0).unwrap().club, "First".to_string());
		assert_eq!(ordered.get(1).unwrap().club, "Second".to_string());
		assert_eq!(ordered.get(2).unwrap().club, "Third".to_string());
	}
}
