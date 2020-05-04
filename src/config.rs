use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Deserializer};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::fmt;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::fs::File;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::io::BufReader;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::fmt;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
	sessions: Vec<Session>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Session {
	name: String,
	link: String,
	#[serde(deserialize_with = "deserialize_weekday")]
	weekday: chrono::Weekday,
	#[serde(deserialize_with = "deserialize_time")]
	start: NaiveTime,
}

impl Config {
	pub fn read<P>(path: P) -> anyhow::Result<Self>
	where
		P: AsRef<Path>,
	{
		let file = File::open(path)?;
		let buf_reader = BufReader::new(file);
		Ok(serde_json::from_reader(buf_reader)?)
	}
}

fn deserialize_weekday<'de, D>(deserializer: D) -> Result<Weekday, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Weekday;

		fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
			formatter.write_str("A capitalized 3-letters weekday")
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			use chrono::Weekday::*;
			match v {
				"Mon" => Ok(Mon),
				"Tue" => Ok(Tue),
				"Wed" => Ok(Wed),
				"Thu" => Ok(Thu),
				"Fri" => Ok(Fri),
				"Sat" => Ok(Sat),
				"Sun" => Ok(Sun),
				unknown => Err(E::custom(format!("unknown weekday {}", unknown))),
			}
		}
	}

	deserializer.deserialize_str(Visitor)
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = NaiveTime;

		fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
			formatter.write_str("A 24-hours time string: HHMM")
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			NaiveTime::parse_from_str(v, "%H%M")
				.map_err(|e| (E::custom(format!("Could not parse time from {}: {}", v, e))))
		}
	}

	deserializer.deserialize_str(Visitor)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn deserialize_config() {
		let config = Config::read("./static/config.json").unwrap();
		assert_eq!(config, Config {
			sessions: vec![
				Session {
					name: "Aston University Jiu Jitsu Club".to_string(),
					link: "https://www.facebook.com/AstonJiuJitsu/".to_string(),
					weekday: Weekday::Mon,
					start: NaiveTime::from_hms(20, 30, 00),
				},
				Session {
					name: "Brighton Jiu Jitsu Club".to_string(),
					link: "https://www.facebook.com/groups/UniversitiesBrightonJitsu/".to_string(),
					weekday: Weekday::Wed,
					start: NaiveTime::from_hms(19, 00, 00),
				}
			]
		})
	}
}
