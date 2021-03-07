#[derive(Debug, Copy, Clone)]
pub enum Event {
	App(AppEvent),
	Window(WindowEvent),
	Keyboard(KeyboardEvent),
	Mouse(MouseEvent),
}

#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
	Close,
	Resize(u32, u32),
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
	Press(Key),
	Release(Key, u32),
}

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
	ButtonPress(MouseButton),
	ButtonRelease(MouseButton),
	Move(i32, i32),
	Scroll(i32, i32),
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Key {
	A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
	CtrlLeft,CtrlRight,ShiftLeft,ShiftRight,AltLeft,AltRight,OSLeft,OSRight,
	CapsLock,NumLock,ScrollLock,
	Digit0,Digit1,Digit2,Digit3,Digit4,Digit5,Digit6,Digit7,Digit8,Digit9,
	Num0,Num1,Num2,Num3,Num4,Num5,Num6,Num7,Num8,Num9,
	F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12,
	PrintScrn,SysRq,Pause,
	Insert,Delete,Home,End,PageUp,PageDown,
	Up,Right,Down,Left,
	BracketLeft,BracketRight,BackSlash,
	Semicolon,Apostrophe,Comma,Period,Slash,BackTick,
	Tab,Enter,Esc,Space,
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
