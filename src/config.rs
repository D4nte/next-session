use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Deserializer};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::fs::File;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::io::BufReader;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::__rt::std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::BufReader;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[derive(Deserialize)]
struct Config {
	sessions: Vec<Session>,
}

#[derive(Deserialize)]
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

fn deserialize_weekday<'de, D>(d: D) -> Result<Weekday, D::Error>
where
	D: Deserializer<'de>,
{
	unimplemented!()
}

fn deserialize_time<'de, D>(d: D) -> Result<NaiveTime, D::Error>
where
	D: Deserializer<'de>,
{
	unimplemented!()
}
