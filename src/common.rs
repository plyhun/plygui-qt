pub use qt_core::variant::{Variant as QVariant};
pub use qt_core::string::{String as QString};
pub use qt_core::cpp_utils::{StaticCast, CppBox};
pub use qt_core::object::{Object as QObject};
pub use qt_core::event::{Type as QEventType, Event as QEvent};
pub use qt_widgets::size_policy::{Policy as QPolicy};
pub use qt_widgets::widget::{Widget as QWidget};
pub use qt_core_custom_events::custom_event_filter::CustomEventFilter;

use std::mem;
use std::ffi::CString;

use plygui_api::{development, ids, layout, types, callbacks, traits};

lazy_static! {
	pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

#[repr(C)]
pub struct QtControlBase {
    pub control_base: development::UiControlCommon, 
    
    pub widget: CppBox<QWidget>,
    pub coords: Option<(i32, i32)>,
    pub measured_size: (u16, u16),
    
    event_callback: CppBox<CustomEventFilter>,
    pub h_resize: Option<callbacks::Resize>,
    
    invalidate: unsafe fn(this: &mut QtControlBase),
}

impl QtControlBase {
	pub fn with_params<F>(widget: *mut QWidget, invalidate: unsafe fn(this: &mut QtControlBase), functions: development::UiMemberFunctions, event_callback: F) -> QtControlBase
			where F: for<'a,'b> FnMut(&'a mut QObject,&'b QEvent) -> bool {
		let mut base = QtControlBase {
        	control_base: development::UiControlCommon {
	        	member_base: development::UiMemberCommon::with_params(types::Visibility::Visible, functions),
		        layout: layout::Attributes {
		            width: layout::Size::MatchParent,
					height: layout::Size::WrapContent,
					gravity: layout::gravity::CENTER_HORIZONTAL | layout::gravity::TOP,
					alignment: layout::Alignment::None,
					..
					Default::default()
	            },
        	},
        	widget: unsafe {CppBox::new(widget)},
        	event_callback: CustomEventFilter::new(event_callback),
        	h_resize: None,
            coords: None,
            measured_size: (0, 0),
            
            invalidate: invalidate
        };
		base.widget.set_size_policy((QPolicy::Ignored, QPolicy::Ignored));
        base.widget.set_minimum_size((1,1));
        unsafe {
        	let filter: *mut QObject = base.event_callback.static_cast_mut() as *mut QObject;
        	let qobject: &mut QObject = base.widget.as_mut().static_cast_mut();
        	qobject.install_event_filter(filter);
        }
        base
	}
	pub fn invalidate(&mut self) {
		unsafe { (self.invalidate)(self) }
	}
    pub unsafe fn on_removed_from_container(&mut self) {
        
    }   
    pub fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.control_base.member_base.visibility != visibility {
            self.control_base.member_base.visibility = visibility;
            
            let widget = self.widget.as_mut();
            let mut sp_retain = widget.size_policy();
            sp_retain.set_retain_size_when_hidden(self.control_base.member_base.visibility != types::Visibility::Gone);
            widget.set_size_policy(&sp_retain);
            widget.set_visible(self.control_base.member_base.visibility == types::Visibility::Visible);
        }
    }
    pub fn visibility(&self) -> types::Visibility {
        self.control_base.member_base.visibility
    }
    pub fn id(&self) -> ids::Id {
        self.control_base.member_base.id
    }
    pub fn parent(&self) -> Option<&types::UiMemberBase> {
        unsafe {
        	let ptr = ((&*self.widget.as_ref().parent_widget()).static_cast() as &QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
            	Some(mem::transmute(ptr))
            } else {
            	None
            }
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        unsafe {
            let ptr = ((&mut *self.widget.as_mut().parent_widget()).static_cast_mut() as &mut QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
            	Some(mem::transmute(ptr))
            } else {
            	None
            }
        }
    }
    pub fn root(&self) -> Option<&types::UiMemberBase> {
        unsafe {
            let ptr = ((&mut *self.widget.as_ref().window()).static_cast_mut() as &mut QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
            	Some(mem::transmute(ptr))
            } else {
            	None
            }
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        unsafe {
            let ptr = ((&mut *self.widget.as_mut().window()).static_cast_mut() as &mut QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
            	Some(mem::transmute(ptr))
            } else {
            	None
            }
        }
    }
}
pub fn cast_uicommon_to_qtcommon_mut<'a>(control: &'a mut development::UiControlCommon) -> &'a mut QtControlBase {
	unsafe {
		mem::transmute(control)
	}
}
pub fn cast_uicommon_to_qtcommon<'a>(control: &'a development::UiControlCommon) -> &'a QtControlBase {
	unsafe {
		mem::transmute(control)
	}
}
fn cast_qobject_mut<'a, T>(object: &'a mut QObject) -> Option<&'a mut T> where T: Sized {
	unsafe {
		let ptr = (&*object).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
		if ptr != 0 {
			Some(::std::mem::transmute(ptr))
		} else {
			None
		}
	}
}
fn cast_qobject<'a, T>(object: &'a QObject) -> Option<&'a T> where T: Sized {
	unsafe {
		let ptr = (&*object).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
		if ptr != 0 {
			Some(::std::mem::transmute(ptr))
		} else {
			None
		}
	}
}
pub fn cast_qobject_to_uimember_mut<'a, T>(object: &'a mut QObject) -> Option<&'a mut T> where T: traits::UiMember + Sized {
	cast_qobject_mut(object)
}
pub fn cast_qobject_to_uimember<'a, T>(object: &'a QObject) -> Option<&'a T> where T: traits::UiMember + Sized {
	cast_qobject(object)
}
pub fn cast_qobject_to_common_mut<'a>(object: &'a mut QObject) -> Option<&'a mut development::UiMemberCommon> {
	cast_qobject_mut(object)
}
pub fn cast_qobject_to_common<'a>(object: &'a QObject) -> Option<&'a development::UiMemberCommon> {
	cast_qobject(object)
}
/*pub unsafe fn downcast_qobject_mut<'a, T>(object: &'a mut QObject) -> &'a mut T where T: Sized {
	mem::transmute(cast_qobject_to_uimember_mut(object).unwrap())
}
pub unsafe fn downcast_qobject<'a, T>(object: &'a QObject) -> &'a T where T: Sized {
	mem::transmute(cast_qobject_to_uimember(object).unwrap())
}*/

#[macro_export]
macro_rules! impl_invalidate {
	($typ: ty) => {
		unsafe fn invalidate_impl(this: &mut common::QtControlBase) {
			use qt_core::cpp_utils::StaticCast;
			use plygui_api::development::UiDrawable;
			
			let parent_widget = this.widget.as_mut().parent_widget();
			if parent_widget.is_null() {
				return;
			}
			if let Some(mparent) = common::cast_qobject_to_common_mut((&mut *parent_widget).static_cast_mut() as &mut ::qt_core::object::Object) {
				let (pw, ph) = mparent.size();
				let this: &mut $typ = ::std::mem::transmute(this);
				let (_,_,changed) = this.measure(pw, ph);
				this.draw(None);		
						
				if mparent.is_control().is_some() {
					let wparent: &mut common::QtControlBase = ::std::mem::transmute(mparent);
					if changed {
						wparent.invalidate();
					} 
				}
			}
		}
	}
}
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