use crossbeam::channel::Sender;
use hazel::{events::Event, log, App};

struct SandboxApp {
	#[allow(dead_code)]
	event_emitter: Sender<Event>,
}

impl App for SandboxApp {
	fn new(event_emitter: Sender<Event>) -> Self {
		Self { event_emitter }
	}

	fn tick(&mut self) {
		log::info("Tick!");
	}
}

fn main() {
	hazel::bootstrap::<SandboxApp>();
}
