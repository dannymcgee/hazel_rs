use crossbeam::channel::Sender;
use glfw::{Context, Glfw, SwapInterval, Window as GLFWWindow, WindowMode};
use std::sync::mpsc;

use crate::events::Event;

#[derive(Debug)]
pub struct Window {
	evt_tx: Sender<Event>,
	glfw: Glfw,
	glfw_rx: mpsc::Receiver<(f64, glfw::WindowEvent)>,
	glfw_window: GLFWWindow,
	title: &'static str,
	width: u32,
	height: u32,
	vsync: bool,
}

impl Window {
	pub(crate) fn new(
		evt_tx: Sender<Event>,
		title: &'static str,
		width: u32,
		height: u32,
		vsync: bool,
	) -> Self {
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		let (mut glfw_window, glfw_rx) = glfw
			.create_window(width, height, title, WindowMode::Windowed)
			.expect("Failed to create GLFW window!");

		glfw_window.make_current();
		glfw_window.set_all_polling(true);

		if vsync {
			glfw.set_swap_interval(SwapInterval::Sync(1));
		} else {
			glfw.set_swap_interval(SwapInterval::None);
		};

		Self {
			evt_tx,
			glfw,
			glfw_rx,
			glfw_window,
			title,
			width,
			height,
			vsync,
		}
	}

	#[allow(unused_must_use)]
	pub(crate) fn tick(&mut self) {
		self.glfw_window.swap_buffers();

		use crate::events::{
			AppEvent::*,
			Event::{
				App as AppEvent, Keyboard as KeyboardEvent, Mouse as MouseEvent,
				Window as WindowEvent,
			},
			KeyboardEvent::*,
			MouseEvent::{ButtonPress, ButtonRelease, Move as MouseMove, Scroll},
			WindowEvent::*,
		};

		let glfw = &mut self.glfw;
		let glfw_window = &mut self.glfw_window;
		let evt_tx = &self.evt_tx;

		glfw.poll_events_unbuffered(|_, (_, evt)| {
			// Convert the GLFW event to a Hazel event
			#[rustfmt::skip]
			let hz_evt = match evt {
				// Window
				glfw::WindowEvent::Pos(x, y)  => WindowEvent(Move(x, y)),
				glfw::WindowEvent::Size(w, h) => WindowEvent(Resize(w, h)),
				glfw::WindowEvent::Close      => WindowEvent(Close),
				glfw::WindowEvent::Focus(_)   => WindowEvent(Focus),
				glfw::WindowEvent::Refresh    => {
					glfw_window.swap_buffers();
					Event::None
				}
				// Mouse
				glfw::WindowEvent::CursorPos(x, y) => MouseEvent(MouseMove(x as u64 as u32, y as u64 as u32)),
				glfw::WindowEvent::Scroll(x, y)    => MouseEvent(Scroll(x, y)),
				glfw::WindowEvent::MouseButton(btn, action, mods) => match action {
					glfw::Action::Release => MouseEvent(ButtonRelease(btn.into(), mods.into())),
					glfw::Action::Press   => MouseEvent(ButtonPress(btn.into(), mods.into())),
					glfw::Action::Repeat  => Event::None,
				},
				// Keyboard
				glfw::WindowEvent::Key(key, _, action, mods) => match action {
					glfw::Action::Release => KeyboardEvent(Release(key.into(), mods.into())),
					glfw::Action::Press   => KeyboardEvent(Press(key.into(), mods.into(), 0)),
					glfw::Action::Repeat  => KeyboardEvent(Press(key.into(), mods.into(), 1)),
				},
				// Ignored
				_ => Event::None,
			};

			// Send the Hazel event over the sync channel
			evt_tx.send(hz_evt);

			// Drop the original event
			None
		});

		self.evt_tx.send(AppEvent(Tick));
	}

	pub fn width(&self) -> u32 {
		self.width
	}

	pub fn height(&self) -> u32 {
		self.height
	}

	pub fn is_vsync(&self) -> bool {
		self.vsync
	}
}
