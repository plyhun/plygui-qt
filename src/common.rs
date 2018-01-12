use super::*;

use std::{ptr, mem, str};
use std::os::raw::c_void;
use std::slice;
use std::ffi::CString;

use plygui_api::{development, ids, layout, types, callbacks};

lazy_static! {
	pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

/*#[repr(C)]
pub struct CocoaControlBase {
    pub control_base: development::UiControlCommon, 
    
    pub control: cocoa_id,
    pub coords: Option<(i32, i32)>,
    pub measured_size: (u16, u16),
    pub h_resize: Option<callbacks::Resize>,
    
    invalidate: unsafe fn(this: &mut CocoaControlBase),
}

impl CocoaControlBase {
	pub fn with_params(invalidate: unsafe fn(this: &mut CocoaControlBase), functions: development::UiMemberFunctions) -> CocoaControlBase {
		CocoaControlBase {
        	control_base: development::UiControlCommon {
	        	member_base: development::UiMemberCommon::with_params(types::Visibility::Visible, functions),
		        layout: development::layout::LayoutBase {
		            width: layout::Size::MatchParent,
					height: layout::Size::WrapContent,
					gravity: layout::gravity::CENTER_HORIZONTAL | layout::gravity::TOP,
					orientation: layout::Orientation::Vertical,
					alignment: layout::Alignment::None,
	            },
        	},
        	control: ptr::null_mut(),
            h_resize: None,
            coords: None,
            measured_size: (0, 0),
            
            invalidate: invalidate
        }
	}
	pub fn invalidate(&mut self) {
		unsafe { (self.invalidate)(self) }
	}
    pub unsafe fn on_removed_from_container(&mut self) {
        self.control.removeFromSuperview();
        msg_send![self.control, dealloc];
        self.control = ptr::null_mut();
    }   
    pub fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.control_base.member_base.visibility != visibility {
            self.control_base.member_base.visibility = visibility;
            unsafe {
                match self.control_base.member_base.visibility {
                    types::Visibility::Visible => {
                        msg_send![self.control, setHidden: NO];
                    }
                    _ => {
                        msg_send![self.control, setHidden: YES];
                    }
                }
            }
            self.invalidate();
        }
    }
    pub fn visibility(&self) -> types::Visibility {
        self.control_base.member_base.visibility
    }
    pub fn id(&self) -> ids::Id {
        self.control_base.member_base.id
    }
    pub fn parent_cocoa_id(&self) -> Option<cocoa_id> {
    	unsafe {
    		parent_cocoa_id(self.control, false)
    	}
    }
    pub fn parent(&self) -> Option<&types::UiMemberBase> {
        unsafe {
            parent_cocoa_id(self.control, false).map(|id| cast_cocoa_id(id).unwrap())
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        unsafe {
            parent_cocoa_id(self.control, false).map(|id| cast_cocoa_id_mut(id).unwrap())
        }
    }
    pub fn root(&self) -> Option<&types::UiMemberBase> {
        unsafe {
            parent_cocoa_id(self.control, true).map(|id| cast_cocoa_id(id).unwrap())
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        unsafe {
            parent_cocoa_id(self.control, true).map(|id| cast_cocoa_id_mut(id).unwrap())
        }
    }
}

#[macro_export]
macro_rules! impl_invalidate {
	($typ: ty) => {
		unsafe fn invalidate_impl(this: &mut common::CocoaControlBase) {
			
		}
	}
}*/
#[macro_export]
macro_rules! impl_is_control {
	($typ: ty) => {
		unsafe fn is_control(this: &::plygui_api::development::UiMemberCommon) -> Option<&::plygui_api::development::UiControlCommon> {
			Some(&::plygui_api::utils::base_to_impl::<$typ>(this).base.control_base)
		}
		unsafe fn is_control_mut(this: &mut ::plygui_api::development::UiMemberCommon) -> Option<&mut ::plygui_api::development::UiControlCommon> {
			Some(&mut ::plygui_api::utils::base_to_impl_mut::<$typ>(this).base.control_base)
		}
	}
}
#[macro_export]
macro_rules! impl_size {
	($typ: ty) => {
		unsafe fn size(this: &::plygui_api::development::UiMemberCommon) -> (u16, u16) {
			::plygui_api::utils::base_to_impl::<$typ>(this).size()
		}
	}
}
#[macro_export]
macro_rules! impl_member_id {
	($mem: expr) => {
		unsafe fn member_id(_: &::plygui_api::development::UiMemberCommon) -> &'static str {
			$mem
		}
	}
}
#[macro_export]
macro_rules! impl_measure {
	($typ: ty) => {
		unsafe fn measure(&mut UiMemberBase, w: u16, h: u16) -> (u16, u16, bool) {
			::plygui_api::utils::base_to_impl::<$typ>(this).measure(w, h)
		}
	}
}
#[macro_export]
macro_rules! impl_draw {
	($typ: ty) => {
		unsafe fn draw(&mut UiMemberBase, coords: Option<(i32, i32)>) {
			::plygui_api::utils::base_to_impl::<$typ>(this).draw(coords)
		}
	}
}