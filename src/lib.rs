#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

pub use qt_core;
pub use qt_core_custom_events;
pub use qt_gui;
pub use qt_widgets;



#[macro_use]
pub mod common;

mod application;
mod button;
mod frame;
mod layout_linear;
mod splitted;
mod window;
mod text;

#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), frame::spawn);
    //registry.register_member(plygui_api::markup::MEMBER_TYPE_SPLITTED.into(), splitted::spawn);
}

pub mod prelude {
    pub use plygui_api::callbacks;
    pub use plygui_api::controls::*;
    pub use plygui_api::ids::*;
    pub use plygui_api::layout;
    pub use plygui_api::types::*;
    pub use plygui_api::utils;

    pub mod imp {
        pub use crate::application::Application;
        pub use crate::button::Button;
        pub use crate::frame::Frame;
        pub use crate::layout_linear::LinearLayout;
        pub use crate::splitted::Splitted;
        pub use crate::window::Window;
        pub use crate::text::Text;
    }
}
