#![warn(
	unused_extern_crates,
	rust_2018_idioms,
	clippy::cast_possible_truncation,
	clippy::cast_sign_loss,
	clippy::fallible_impl_from,
	clippy::cast_precision_loss,
	clippy::cast_possible_wrap,
	clippy::print_stdout,
	clippy::dbg_macro
)]
#![forbid(unsafe_code)]

use crate::{schedule::Schedule, session::Session};
use chrono::{NaiveTime, Weekday};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod config;
mod schedule;
mod session;

struct Model {
	sessions: Vec<Session>,
}

impl Model {
	pub fn sort(&mut self) {
		Session::sort(&mut self.sessions)
	}
}

impl Component for Model {
	type Message = ();
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		let mut model = Model::default();
		model.sort();
		model
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
		self.sort();
		true
	}

	fn change(&mut self, _: Self::Properties) -> ShouldRender {
		// Should only return "true" if new properties are different to
		// previously received properties.
		// This component has no properties so we will always return "false".
		false
	}

	fn view(&self) -> Html {
		html! {
			<div class="container">
				{ for self.sessions.iter().map(|s| s.view() ) }
			</div>
		}
	}
}

impl Default for Model {
	fn default() -> Self {
		use Weekday::*;
		Self {
			sessions: vec![
				Session {
					club: "UTS Jitsu".to_string(),
					schedule: Schedule::new(Wed, NaiveTime::from_hms(19, 15, 00), 10),
				},
				Session {
					club: "London Green +".to_string(),
					schedule: Schedule::new(Sat, NaiveTime::from_hms(10, 45, 00), 1),
				},
				Session {
					club: "Aston University Jiu Jitsu Club".to_string(),
					schedule: Schedule::new(Mon, NaiveTime::from_hms(20, 30, 00), 1),
				},
			],
		}
	}
}

#[wasm_bindgen(start)]
pub fn run_app() {
	App::<Model>::new().mount_to_body();
}
