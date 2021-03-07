pub use crossbeam::channel::{Receiver, Sender};
use std::convert::From;

#[derive(Debug, Copy, Clone)]
pub enum Event {
	App(AppEvent),
	Window(WindowEvent),
	Keyboard(KeyboardEvent),
	Mouse(MouseEvent),
	None,
}

#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
	Close,
	Resize(i32, i32),
	Focus,
	Blur,
	Move(i32, i32),
}

#[derive(Debug, Copy, Clone)]
pub enum AppEvent {
	Tick,
	Update,
	Render,
}

#[derive(Debug, Copy, Clone)]
pub enum KeyboardEvent {
	Press(Key, Modifiers, u32),
	Release(Key, Modifiers),
}

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
	ButtonPress(MouseButton, Modifiers),
	ButtonRelease(MouseButton, Modifiers),
	Move(u32, u32),
	Scroll(f64, f64),
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Key {
	A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
	CtrlLeft,CtrlRight,ShiftLeft,ShiftRight,AltLeft,AltRight,OSLeft,OSRight,Menu,
	CapsLock,NumLock,ScrollLock,
	Digit0,Digit1,Digit2,Digit3,Digit4,Digit5,Digit6,Digit7,Digit8,Digit9,
	Num0,Num1,Num2,Num3,Num4,Num5,Num6,Num7,Num8,Num9,
	NumDecimal,NumDivide,NumMultiply,NumMinus,NumPlus,NumEnter,NumEqual,
	F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,F13,F14,F15,F16,F17,F18,F19,F20,F21,F22,F23,F24,F25,
	PrintScreen,SysRq,Pause,
	Insert,Delete,Home,End,PageUp,PageDown,
	Up,Right,Down,Left,
	BracketLeft,BracketRight,Backslash,
	Semicolon,Apostrophe,Comma,Period,Slash,BackTick,Minus,Equal,
	Tab,Enter,Esc,Space,Backspace,
	World1,World2,
	Unknown,
}

bitflags! {
	pub struct Modifiers: std::os::raw::c_int {
		const SHIFT  = glfw::ffi::MOD_SHIFT;
		const CTRL   = glfw::ffi::MOD_CONTROL;
		const ALT    = glfw::ffi::MOD_ALT;
		const SUPER  = glfw::ffi::MOD_SUPER;
		const CAPSLK = glfw::ffi::MOD_CAPS_LOCK;
		const NUMLK  = glfw::ffi::MOD_NUM_LOCK;
	}
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
	Primary,
	Secondary,
	Middle,
	Back,
	Forward,
	Button6,
	Button7,
	Button8,
	Button9,
	Button10,
	Button11,
	Button12,
	Button13,
	Button14,
	Button15,
	Button16,
	Button17,
	Button18,
	Button19,
	Button20,
}

#[rustfmt::skip]
impl From<glfw::Key> for Key {
	fn from(key: glfw::Key) -> Self {
		use Key::*;

		match key {
			glfw::Key::Space        => Space,
			glfw::Key::Apostrophe   => Apostrophe,
			glfw::Key::Comma        => Comma,
			glfw::Key::Minus        => Minus,
			glfw::Key::Period       => Period,
			glfw::Key::Slash        => Slash,
			glfw::Key::Num0         => Num0,
			glfw::Key::Num1         => Num1,
			glfw::Key::Num2         => Num2,
			glfw::Key::Num3         => Num3,
			glfw::Key::Num4         => Num4,
			glfw::Key::Num5         => Num5,
			glfw::Key::Num6         => Num6,
			glfw::Key::Num7         => Num7,
			glfw::Key::Num8         => Num8,
			glfw::Key::Num9         => Num9,
			glfw::Key::Semicolon    => Semicolon,
			glfw::Key::Equal        => Equal,
			glfw::Key::A            => A,
			glfw::Key::B            => B,
			glfw::Key::C            => C,
			glfw::Key::D            => D,
			glfw::Key::E            => E,
			glfw::Key::F            => F,
			glfw::Key::G            => G,
			glfw::Key::H            => H,
			glfw::Key::I            => I,
			glfw::Key::J            => J,
			glfw::Key::K            => K,
			glfw::Key::L            => L,
			glfw::Key::M            => M,
			glfw::Key::N            => N,
			glfw::Key::O            => O,
			glfw::Key::P            => P,
			glfw::Key::Q            => Q,
			glfw::Key::R            => R,
			glfw::Key::S            => S,
			glfw::Key::T            => T,
			glfw::Key::U            => U,
			glfw::Key::V            => V,
			glfw::Key::W            => W,
			glfw::Key::X            => X,
			glfw::Key::Y            => Y,
			glfw::Key::Z            => Z,
			glfw::Key::LeftBracket  => BracketLeft,
			glfw::Key::Backslash    => Backslash,
			glfw::Key::RightBracket => BracketRight,
			glfw::Key::GraveAccent  => BackTick,
			glfw::Key::World1       => World1,
			glfw::Key::World2       => World2,
			glfw::Key::Escape       => Esc,
			glfw::Key::Enter        => Enter,
			glfw::Key::Tab          => Tab,
			glfw::Key::Backspace    => Backspace,
			glfw::Key::Insert       => Insert,
			glfw::Key::Delete       => Delete,
			glfw::Key::Right        => Right,
			glfw::Key::Left         => Left,
			glfw::Key::Down         => Down,
			glfw::Key::Up           => Up,
			glfw::Key::PageUp       => PageUp,
			glfw::Key::PageDown     => PageDown,
			glfw::Key::Home         => Home,
			glfw::Key::End          => End,
			glfw::Key::CapsLock     => CapsLock,
			glfw::Key::ScrollLock   => ScrollLock,
			glfw::Key::NumLock      => NumLock,
			glfw::Key::PrintScreen  => PrintScreen,
			glfw::Key::Pause        => Pause,
			glfw::Key::F1           => F1,
			glfw::Key::F2           => F2,
			glfw::Key::F3           => F3,
			glfw::Key::F4           => F4,
			glfw::Key::F5           => F5,
			glfw::Key::F6           => F6,
			glfw::Key::F7           => F7,
			glfw::Key::F8           => F8,
			glfw::Key::F9           => F9,
			glfw::Key::F10          => F10,
			glfw::Key::F11          => F11,
			glfw::Key::F12          => F12,
			glfw::Key::F13          => F13,
			glfw::Key::F14          => F14,
			glfw::Key::F15          => F15,
			glfw::Key::F16          => F16,
			glfw::Key::F17          => F17,
			glfw::Key::F18          => F18,
			glfw::Key::F19          => F19,
			glfw::Key::F20          => F20,
			glfw::Key::F21          => F21,
			glfw::Key::F22          => F22,
			glfw::Key::F23          => F23,
			glfw::Key::F24          => F24,
			glfw::Key::F25          => F25,
			glfw::Key::Kp0          => Digit0,
			glfw::Key::Kp1          => Digit1,
			glfw::Key::Kp2          => Digit2,
			glfw::Key::Kp3          => Digit3,
			glfw::Key::Kp4          => Digit4,
			glfw::Key::Kp5          => Digit5,
			glfw::Key::Kp6          => Digit6,
			glfw::Key::Kp7          => Digit7,
			glfw::Key::Kp8          => Digit8,
			glfw::Key::Kp9          => Digit9,
			glfw::Key::KpDecimal    => NumDecimal,
			glfw::Key::KpDivide     => NumDivide,
			glfw::Key::KpMultiply   => NumMultiply,
			glfw::Key::KpSubtract   => NumMinus,
			glfw::Key::KpAdd        => NumPlus,
			glfw::Key::KpEnter      => NumEnter,
			glfw::Key::KpEqual      => NumEqual,
			glfw::Key::LeftShift    => ShiftLeft,
			glfw::Key::LeftControl  => CtrlLeft,
			glfw::Key::LeftAlt      => AltLeft,
			glfw::Key::LeftSuper    => OSLeft,
			glfw::Key::RightShift   => ShiftRight,
			glfw::Key::RightControl => CtrlRight,
			glfw::Key::RightAlt     => AltRight,
			glfw::Key::RightSuper   => OSRight,
			glfw::Key::Menu         => Menu,
			glfw::Key::Unknown      => Unknown,
		}
	}
}

impl From<glfw::Modifiers> for Modifiers {
	fn from(mods: glfw::Modifiers) -> Self {
		Modifiers::from_bits(mods.bits()).unwrap()
	}
}

impl From<glfw::MouseButton> for MouseButton {
	fn from(button: glfw::MouseButton) -> Self {
		use MouseButton::*;

		match button {
			glfw::MouseButton::Button1 => Primary,
			glfw::MouseButton::Button2 => Secondary,
			glfw::MouseButton::Button3 => Middle,
			glfw::MouseButton::Button4 => Back,
			glfw::MouseButton::Button5 => Forward,
			glfw::MouseButton::Button6 => Button6,
			glfw::MouseButton::Button7 => Button7,
			glfw::MouseButton::Button8 => Button8,
		}
	}
}
