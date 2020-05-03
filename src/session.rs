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
		Session {
			club: "UTS Jitsu".to_string(),
			schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(19, 15, 00), 10),
		}
	}
}
