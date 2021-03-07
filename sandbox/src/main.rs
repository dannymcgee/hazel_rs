use hazel::{
	events::{Event, Key, Modifiers, MouseButton, Sender},
	log, App,
};

struct SandboxApp {
	#[allow(dead_code)]
	event_emitter: Sender<Event>,
}

impl App for SandboxApp {
	fn new(event_emitter: Sender<Event>) -> Self {
		Self { event_emitter }
	}

	fn tick(&mut self) {
		// log::info("Tick!");
	}

	fn on_window_resize(&mut self, w: i32, h: i32) {
		log::debug(&format!("Window resize! w: {}, h: {}", w, h));
	}

	fn on_window_move(&mut self, x: i32, y: i32) {
		log::debug(&format!("Window move! x: {}, y: {}", x, y));
	}

	fn on_keydown(&mut self, key: Key, mods: Modifiers, repeats: u32) {
		if mods.is_empty() {
			log::debug(&format!("Keydown! {:?} x{}", key, repeats));
		} else {
			log::debug(&format!("Keydown: {:?} + {:?} x{}", mods, key, repeats));
		}
	}

	fn on_keyup(&mut self, key: Key, mods: Modifiers) {
		if mods.is_empty() {
			log::debug(&format!("Keyup! {:?}", key));
		} else {
			log::debug(&format!("Keyup! {:?} + {:?}", mods, key));
		}
	}

	fn on_mousedown(&mut self, button: MouseButton, mods: Modifiers) {
		if mods.is_empty() {
			log::debug(&format!("Mousedown! {:?}", button));
		} else {
			log::debug(&format!("Mousedown! {:?} + {:?}", mods, button));
		}
	}

	fn on_mouseup(&mut self, button: MouseButton, mods: Modifiers) {
		if mods.is_empty() {
			log::debug(&format!("Mouseup! {:?}", button));
		} else {
			log::debug(&format!("Mouseup! {:?} + {:?}", mods, button));
		}
	}

	fn on_mousemove(&mut self, x: u32, y: u32) {
		log::debug(&format!("Mouse move! x: {}, y: {}", x, y));
	}

	fn on_scroll(&mut self, dx: f64, dy: f64) {
		log::debug(&format!("Scroll! dx: {}, dy: {}", dx, dy));
	}
}

fn main() {
	hazel::bootstrap::<SandboxApp>();
}
