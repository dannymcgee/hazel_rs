#![allow(dead_code)]

use ansi_term::Color::*;
use chrono::Local;
use std::process;

pub(crate) enum Level {
	Debug,
	Info,
	Okay,
	Warn,
	Error,
	Fatal,
}

enum Source {
	Core,
	Client,
}

pub fn debug(msg: &str) {
	log_client(Level::Debug, msg);
}
pub fn okay(msg: &str) {
	log_client(Level::Okay, msg);
}
pub fn info(msg: &str) {
	log_client(Level::Info, msg);
}
pub fn warn(msg: &str) {
	log_client(Level::Warn, msg);
}
pub fn error(msg: &str) {
	log_client(Level::Error, msg);
}
pub fn fatal(msg: &str) {
	log_client(Level::Fatal, msg);
	process::exit(1);
}

pub(crate) fn core_debug(msg: &str) {
	log_core(Level::Debug, msg);
}
pub(crate) fn core_okay(msg: &str) {
	log_core(Level::Okay, msg);
}
pub(crate) fn core_info(msg: &str) {
	log_core(Level::Info, msg);
}
pub(crate) fn core_warn(msg: &str) {
	log_core(Level::Warn, msg);
}
pub(crate) fn core_error(msg: &str) {
	log_core(Level::Error, msg);
}
pub(crate) fn core_fatal(msg: &str) {
	log_core(Level::Fatal, msg);
	process::exit(1);
}

fn log_core(level: Level, msg: &str) {
	match level {
		Level::Error => eprintln!("{} {}", timestamp(level, Source::Core), msg),
		_ => println!("{} {}", timestamp(level, Source::Core), msg),
	}
}
fn log_client(level: Level, msg: &str) {
	match level {
		Level::Error => eprintln!("{} {}", timestamp(level, Source::Client), msg),
		_ => println!("{} {}", timestamp(level, Source::Client), msg),
	}
}

fn timestamp(level: Level, source: Source) -> String {
	let time = Local::now().format("%H:%M:%S%.3f").to_string();

	let source_tag = match source {
		Source::Core => format!(" {}:", Fixed(3).bold().paint("hazel")),
		_ => "".to_string(),
	};

	#[rustfmt::skip]
	let style = match level {
		Level::Debug => Fixed(3).bold(),
		Level::Info  => Fixed(4).bold(),
		Level::Okay  => Fixed(2).reverse().bold(),
		Level::Warn  => Fixed(3).reverse().bold(),
		Level::Error => Fixed(9).reverse().bold(),
		Level::Fatal => Fixed(1).reverse().bold(),
	};

	#[rustfmt::skip]
	let level_tag = match level {
		Level::Debug => " DEBG ",
		Level::Info  => " INFO ",
		Level::Okay  => " OKAY ",
		Level::Warn  => " WARN ",
		Level::Error => " ERRR ",
		Level::Fatal => " FATL ",
	};

	format!(
		"{} {}{}",
		Fixed(8).paint(time),
		style.paint(level_tag),
		source_tag,
	)
}
