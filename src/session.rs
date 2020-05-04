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

	// noinspection RsTypeCheck
	fn view(&self) -> Html {
		web_sys::console::log_1(&"view on session".into());
		let club = self.club.clone();
		let time = utc_to_local(&self.schedule.next())
			.format("%A %H:%M")
			.to_string();

		let text = move || -> Html {
			if self.schedule.unverified() {
				html! {
					<p class="card-text">{ time } <svg class="bi bi-question-circle ml-1" width="1em" height="1em" viewBox="0 0 20 20" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
				<path fill-rule="evenodd" d="M8 15A7 7 0 108 1a7 7 0 000 14zm0 1A8 8 0 108 0a8 8 0 000 16z" clip-rule="evenodd"/>
					<path d="M5.25 6.033h1.32c0-.781.458-1.384 1.36-1.384.685 0 1.313.343 1.313 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.007.463h1.307v-.355c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.326 0-2.786.647-2.754 2.533zm1.562 5.516c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
					</svg>
					</p>
					}
			} else {
				html! {
					<p class="card-text">{ time }</p>
				}
			}
		};

		html! {
		<div class="card bg-light mt-3 mb-3" style="">
			<div class="card-body">
				<h5 class="card-title">{ club }</h5>
				{ text() }
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
			schedule: Schedule::new(Weekday::Wed, Some(NaiveTime::from_hms(19, 15, 00)), 10),
		}
	}
}

impl Session {
	pub fn sort(sessions: &mut Vec<Session>) {
		Self::sort_from_time(sessions, Utc::now())
	}

	pub fn sort_from_time(sessions: &mut Vec<Session>, time: DateTime<Utc>) {
		sessions.sort_by(|a, b| {
			a.schedule
				.time_to_next_from_time(time)
				.cmp(&b.schedule.time_to_next_from_time(time))
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{NaiveTime, TimeZone, Weekday};

	#[test]
	fn schedules_are_ordered_chronologically() {
		let mut sessions = vec![
			Session {
				club: "Second".to_string(),
				schedule: Schedule::new(Weekday::Wed, Some(NaiveTime::from_hms(19, 00, 00)), 0),
			},
			Session {
				club: "First".to_string(),
				schedule: Schedule::new(Weekday::Wed, Some(NaiveTime::from_hms(20, 00, 00)), 3),
			},
			Session {
				club: "Third".to_string(),
				schedule: Schedule::new(Weekday::Mon, Some(NaiveTime::from_hms(10, 00, 00)), 1),
			},
		];

		Session::sort_from_time(
			&mut sessions,
			Utc.isoywd(2020, 2, Weekday::Tue).and_hms(12, 00, 00),
		);

		assert_eq!(sessions.get(0).unwrap().club, "First".to_string());
		assert_eq!(sessions.get(1).unwrap().club, "Second".to_string());
		assert_eq!(sessions.get(2).unwrap().club, "Third".to_string());
	}
}
