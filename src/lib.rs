#![warn(
	unused_extern_crates,
	missing_debug_implementations,
	missing_copy_implementations,
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

use crate::session::{Schedule, Session};
use chrono::{NaiveTime, Weekday};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod session;

struct Model {
	sessions: Vec<Session>,
}

impl Component for Model {
	type Message = ();
	type Properties = ();
	fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
		Model::default()
	}

	fn update(&mut self, _: Self::Message) -> ShouldRender {
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
			<>
				{ for self.sessions.iter().map(|s| s.view() ) }
			</>
		}
	}
}

impl Default for Model {
	fn default() -> Self {
		Self {
			sessions: vec![
				Session {
					club: "UTS Jitsu".to_string(),
					schedule: Schedule::new(Weekday::Wed, NaiveTime::from_hms(19, 15, 00)),
				},
				Session {
					club: "London Green +".to_string(),
					schedule: Schedule::new(Weekday::Sat, NaiveTime::from_hms(11, 15, 00)),
				},
			],
		}
	}
}

#[wasm_bindgen(start)]
pub fn run_app() {
	App::<Model>::new().mount_to_body();
}
