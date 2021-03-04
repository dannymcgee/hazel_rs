use hazel::{log, App};

struct SandboxApp;

impl App for SandboxApp {
	fn run(&mut self, _delta_time: f64) {
		log::debug("Hmmm, what seems to be the problem here...");
		log::info("Tick!");
		log::okay("Success!");
		log::warn("Danger, Will Robinson!");
		log::error("Error! Error!");
		log::fatal("X{");
	}
}

fn main() {
	hazel::bootstrap(&mut SandboxApp);
}
