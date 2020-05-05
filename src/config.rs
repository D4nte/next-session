use crate::std::{fmt, fs::File, io::BufReader, path::Path};
use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
	pub sessions: Vec<Session>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Session {
	pub name: String,
	pub link: String,
	#[serde(deserialize_with = "deserialize_weekday")]
	pub weekday: Weekday,
	#[serde(default)]
	#[serde(deserialize_with = "deserialize_time")]
	pub start: Option<NaiveTime>,
}

impl Config {
	#[allow(dead_code)]
	pub fn read<P>(path: P) -> anyhow::Result<Self>
	where
		P: AsRef<Path>,
	{
		let file = File::open(path)?;
		let buf_reader = BufReader::new(file);
		Ok(serde_json::from_reader(buf_reader)?)
	}

	pub fn load(data: &str) -> anyhow::Result<Self> {
		Ok(serde_json::from_str(data)?)
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

fn deserialize_time<'de, D>(deserializer: D) -> Result<Option<NaiveTime>, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visitor;

	impl<'de> serde::de::Visitor<'de> for Visitor {
		type Value = Option<NaiveTime>;

		fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
			formatter.write_str("A 24-hours time string: HHMM")
		}

		fn visit_none<E>(self) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			Ok(None)
		}

		fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
		{
			NaiveTime::parse_from_str(v, "%H%M")
				.map_err(|e| (E::custom(format!("Could not parse time from {}: {}", v, e))))
				.map(Some)
		}
	}

	deserializer.deserialize_any(Visitor)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::data;

	#[test]
	fn deserialize_config_sample() {
		let content = r#"
{
  "sessions": [
    {
      "name": "Aston University Jiu Jitsu Club",
      "link": "https://www.facebook.com/AstonJiuJitsu/",
      "weekday": "Mon",
      "start": "2030"
    }, {
      "name": "Brighton Jiu Jitsu Club",
      "link": "https://www.facebook.com/groups/UniversitiesBrightonJitsu/",
      "weekday": "Wed",
      "start": "1900"
    }
  ]
}
		"#;
		let config: Config = serde_json::from_str(content).unwrap();
		assert_eq!(config, Config {
			sessions: vec![
				Session {
					name: "Aston University Jiu Jitsu Club".to_string(),
					link: "https://www.facebook.com/AstonJiuJitsu/".to_string(),
					weekday: Weekday::Mon,
					start: Some(NaiveTime::from_hms(20, 30, 00)),
				},
				Session {
					name: "Brighton Jiu Jitsu Club".to_string(),
					link: "https://www.facebook.com/groups/UniversitiesBrightonJitsu/".to_string(),
					weekday: Weekday::Wed,
					start: Some(NaiveTime::from_hms(19, 00, 00)),
				}
			]
		})
	}

	#[test]
	fn deserialize_whole_config() {
		let _: Config = serde_json::from_str(data::DATA).unwrap();
	}
}
