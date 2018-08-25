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

pub use self::application::Application;
pub use self::button::Button;
pub use self::frame::Frame;
pub use self::layout_linear::LinearLayout;
pub use self::splitted::Splitted;
pub use self::window::Window;

#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), frame::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), splitted::spawn);
}
