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
#![recursion_limit = "256"]

use crate::model::Model;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod config;
mod data;
mod model;
mod schedule;
mod session;

#[wasm_bindgen(start)]
pub fn run_app() {
	App::<Model>::new().mount_to_body();
}
