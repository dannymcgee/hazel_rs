struct SandboxApp;

impl hazel::App for SandboxApp {
	fn run(&mut self, delta_time: f64) {
		println!("Tick! {}", delta_time);
	}
}

fn main() {
	hazel::bootstrap(&mut SandboxApp);
}
