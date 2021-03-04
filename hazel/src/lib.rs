use std::time::Instant;

pub trait App {
	fn run(&mut self, delta_time: f64);
}

struct Hazel<'a, T: App> {
	last_tick: Instant,
	app: &'a mut T,
}
impl<'a, T> Hazel<'a, T>
where
	T: App,
{
	fn new(app: &'a mut T) -> Hazel<'a, T> {
		Hazel {
			app,
			last_tick: Instant::now(),
		}
	}

	fn run(&mut self) {
		loop {
			let now = Instant::now();
			let delta_time = (now - self.last_tick).as_nanos() as f64 / 1_000_000.0;

			self.app.run(delta_time);

			self.last_tick = now;
		}
	}
}

pub fn bootstrap<T: App>(app: &mut T) {
	Hazel::new(app).run();
}
