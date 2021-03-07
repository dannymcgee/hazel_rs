use std::{process, thread};

use events::{
	AppEvent, Event, Key, KeyboardEvent, Modifiers, MouseButton, MouseEvent, WindowEvent,
};

pub mod events;
pub mod log;

#[macro_use]
extern crate bitflags;

#[cfg_attr(target_family = "windows", path = "platform/windows/window.rs")]
pub mod window;
use window::Window;

pub trait App {
	fn new(tx: events::Sender<Event>) -> Self;
	fn tick(&mut self) {}
	fn on_update(&mut self) {}
	fn on_render(&mut self) {}
	fn on_window_resize(&mut self, _w: i32, _h: i32) {}
	fn on_window_move(&mut self, _x: i32, _y: i32) {}
	fn on_window_close(&mut self) {
		process::exit(0);
	}
	fn on_keydown(&mut self, _key: Key, _modifiers: Modifiers, _repeat_count: u32) {}
	fn on_keyup(&mut self, _key: Key, _modifiers: Modifiers) {}
	fn on_mousedown(&mut self, _button: MouseButton, _modifiers: Modifiers) {}
	fn on_mouseup(&mut self, _button: MouseButton, _modifiers: Modifiers) {}
	fn on_mousemove(&mut self, _x: u32, _y: u32) {}
	fn on_scroll(&mut self, _dx: f64, _dy: f64) {}
}

pub fn bootstrap<T: 'static + App + Send>() {
	let (tx, rx) = crossbeam::channel::bounded(0);

	let mut app = T::new(tx.clone());
	let mut window = Window::new(tx, "Hello, world!", 1440, 900, true);

	// Event dispatcher
	thread::spawn(move || loop {
		use AppEvent::*;
		use Event::*;
		use KeyboardEvent::*;
		use MouseEvent::{Move as MouseMove, *};
		use WindowEvent::{Move as WindowMove, *};

		if let Ok(evt) = rx.recv() {
			match evt {
				App(Tick) => app.tick(),
				Window(evt) => match evt {
					Resize(w, h) => app.on_window_resize(w, h),
					WindowMove(x, y) => app.on_window_move(x, y),
					Close => app.on_window_close(),
					_ => {}
				},
				Keyboard(evt) => match evt {
					Press(key, mods, repeat) => app.on_keydown(key, mods, repeat),
					Release(key, mods) => app.on_keyup(key, mods),
				},
				Mouse(evt) => match evt {
					ButtonPress(button, mods) => app.on_mousedown(button, mods),
					ButtonRelease(button, mods) => app.on_mouseup(button, mods),
					MouseMove(x, y) => app.on_mousemove(x, y),
					Scroll(dx, dy) => app.on_scroll(dx, dy),
				},
				_ => {}
			};
		}
	});

	loop {
		window.tick();
	}
}
