use crate::config::action;

pub enum GestureVersion {
	V1(GestureV1)
}

pub enum ModeVersion {
	V1(ModeV1)
}

pub struct GestureV1 {
	/// The direction needed for the gesture to activate
	direction: Direction,
	/// The mode needed for activation
	mode: ModeV1,
	/// The action to perform
	action: action::ActionV1
}

pub enum Direction {
	Left,
	Right,
	Up,
	Down,
}

pub enum ModeV1 {
	/// Do nothing
	NoPress,
	/// Do something when the gesture button is released
	OnRelease,
	/// Do something every n pixels
	OnInterval,
	/// Do something once after n pixels
	OnThreshold,
}
