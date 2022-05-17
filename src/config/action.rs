use crate::config::gestures;

pub enum ActionTypeVersion {
	V1(ActionTypeV1)
}

pub enum ActionTypeV1 {
	None,
	Keypress,
	Gestures,
	ToggleSmartShift,
	CycleDPI,
	ChangeDPI,
	ChangeHost,
}

pub struct ActionV1 {
	/// The type of action
	r#type: ActionTypeV1,
	
	// Type specific fields
	/// Run the array of Keys, Refer to https://github.com/torvalds/linux/blob/master/include/uapi/linux/input-event-codes.h
	keys: Option<Vec<String>>,
	/// The array of DPIs to change to
	dpi: Option<Vec<u32>>,
	///	Gestures
	gestures: Option<Vec<gestures::GestureV1>>
}