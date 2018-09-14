#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

extern crate libc;
extern crate qt_core;
extern crate qt_core_custom_events;
extern crate qt_gui;
extern crate qt_widgets;

#[macro_use]
pub mod common;

mod application;
mod button;
mod frame;
mod layout_linear;
mod splitted;
mod window;

#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), frame::spawn);
    //registry.register_member(plygui_api::markup::MEMBER_TYPE_SPLITTED.into(), splitted::spawn);
}

pub mod prelude {
	pub use plygui_api::controls::*;
	pub use plygui_api::ids::*;
	pub use plygui_api::types::*;
	pub use plygui_api::callbacks;
	pub use plygui_api::layout;
	pub use plygui_api::utils; 
	
	pub mod imp {
		pub use ::application::Application;
		pub use ::window::Window;
		pub use ::button::Button;
		pub use ::layout_linear::LinearLayout;
		pub use ::frame::Frame;
		pub use ::splitted::Splitted;
	}
}
