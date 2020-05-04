use crate::{config::Config, data, schedule::Schedule, session::Session};
use yew::prelude::*;

pub struct Model {
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
		let config = Config::load(data::DATA).expect("Config file could not be read");
		let mut model: Model = config.into();
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

impl From<Config> for Model {
	fn from(config: Config) -> Self {
		let sessions = config
			.sessions
			.into_iter()
			.map(|session| {
				Session {
					club: session.name,
					// Assume all timings are BST for now.
					schedule: Schedule::new(session.weekday, session.start, 1),
				}
			})
			.collect();

		Model { sessions }
	}
}
