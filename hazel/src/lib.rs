use std::{thread, time::Duration};

use crossbeam::channel::{self as chan, Sender};
use events::{AppEvent, Event, Key, KeyboardEvent, MouseButton, MouseEvent, WindowEvent};

pub mod events;
pub mod log;

pub trait App {
	fn new(event_emitter: Sender<Event>) -> Self;
	fn tick(&mut self) {}
	fn on_update(&mut self) {}
	fn on_render(&mut self) {}
	fn on_window_resize(&mut self, _w: u32, _h: u32) {}
	fn on_window_move(&mut self, _dx: i32, _dy: i32) {}
	fn on_keydown(&mut self, _key: Key) {}
	fn on_keyup(&mut self, _key: Key, _repeat_count: u32) {}
	fn on_mousedown(&mut self, _button: MouseButton) {}
	fn on_mouseup(&mut self, _button: MouseButton) {}
	fn on_mousemove(&mut self, _dx: i32, _dy: i32) {}
	fn on_scroll(&mut self, _dx: i32, _dy: i32) {}
}

pub fn bootstrap<T: 'static + App + Send>() {
	let (tx, rx) = chan::unbounded();

	let app_tx = tx.clone();
	let mut app = T::new(app_tx);

	loop {
		use AppEvent::*;
		use Event::*;
		use KeyboardEvent::*;
		use MouseEvent::{Move as MouseMove, *};
		use WindowEvent::{Move as WindowMove, *};

		if let Ok(evt) = rx.try_recv() {
			match evt {
				App(evt) => match evt {
					Tick => app.tick(),
					Update => app.on_update(),
					Render => app.on_render(),
				},
				Window(evt) => match evt {
					Resize(w, h) => app.on_window_resize(w, h),
					WindowMove(dx, dy) => app.on_window_move(dx, dy),
					_ => {}
				},
				Keyboard(evt) => match evt {
					Press(key) => app.on_keydown(key),
					Release(key, repeat_count) => app.on_keyup(key, repeat_count),
				},
				Mouse(evt) => match evt {
					ButtonPress(button) => app.on_mousedown(button),
					ButtonRelease(button) => app.on_mouseup(button),
					MouseMove(dx, dy) => app.on_mousemove(dx, dy),
					Scroll(dx, dy) => app.on_scroll(dx, dy),
				},
			}
		}

		if tx.send(App(AppEvent::Tick)).is_ok() {
			thread::sleep(Duration::from_nanos(16_666_667))
		} else {
			break;
		}
	}
}
