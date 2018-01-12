use super::*;
use super::common::*;

use qt_core::string::String;
use qt_core::variant::Variant;
use qt_core::event::{Type, Event};
use qt_core_custom_events::custom_event_filter::CustomEventFilter;
use qt_core::cpp_utils::{StaticCast, CppBox};
use qt_core::object::Object;
use qt_widgets::main_window::{MainWindow as QMainWindow};
use qt_widgets::application::{Application as QApplication};

use plygui_api::{development, ids, types, callbacks};
use plygui_api::traits::{UiControl, UiWindow, UiSingleContainer, UiMember, UiContainer};
use plygui_api::members::MEMBER_ID_WINDOW;

#[repr(C)]
pub struct Window {
	base: development::UiMemberCommon,
	
    pub(crate) window: CppBox<QMainWindow>,
    //pub(crate) container: id,
    
    child: Option<Box<UiControl>>,
    h_resize: Option<callbacks::Resize>,
    
    
    filter: CppBox<CustomEventFilter>,
}

impl Window {
    pub(crate) fn new(
                      title: &str,
                      start_size: types::WindowStartSize,
                      has_menu: bool)
                      -> Box<Window> {
        let mut window = Box::new(Window {
	        base: development::UiMemberCommon::with_params(
	            types::Visibility::Visible,
                development::UiMemberFunctions {
                    fn_member_id: member_id,
                    fn_is_control: is_control,
                    fn_is_control_mut: is_control_mut,
                    fn_size: size,
                }
            ),
	        window: QMainWindow::new(),
	        child: None,
	        h_resize: None,
	        
	        filter: CustomEventFilter::new(event_handler),
        });
        unsafe {
        	let ptr = window.as_ref() as *const _ as u64;
        	(window.window.as_mut().static_cast_mut() as &mut Object).set_property(PROPERTY.as_ptr() as *const i8, &Variant::new0(ptr));
        }
        window.window.set_window_title(&String::from_std_str(title));
        window.window.resize(match start_size {
	        types::WindowStartSize::Exact(w, h) => {
		        (w as i32, h as i32)
	        }
	        types::WindowStartSize::Fullscreen => {
		        let screen = unsafe { (*QApplication::desktop()).screen_geometry(()) };
		        (screen.width(), screen.height())
	        }
        });
        unsafe {
        	let filter: *mut Object = window.filter.static_cast_mut() as *mut Object;
        	let qobject: &mut Object = window.window.as_mut().static_cast_mut();
        	qobject.install_event_filter(filter);
        }
        window.window.show();
        window
    }
}

impl UiWindow for Window {
	fn as_single_container(&self) -> &UiSingleContainer {
		self
	}
	fn as_single_container_mut(&mut self) -> &mut UiSingleContainer {
		self
	}
}

impl UiSingleContainer for Window {
	fn set_child(&mut self, mut child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
        unimplemented!()
    }
    fn child(&self) -> Option<&UiControl> {
        unimplemented!()
    }
    fn child_mut(&mut self) -> Option<&mut UiControl> {
        unimplemented!()
    }
    fn as_container(&self) -> &UiContainer {
    	unimplemented!()
    }
	fn as_container_mut(&mut self) -> &mut UiContainer {
		unimplemented!()
	}
}

impl UiContainer for Window {
    fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut UiControl> {
        /*if self.id() == id_ {
			return Some(self);
		} else*/
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id_);
            }
        }
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&UiControl> {
        /*if self.id() == id_ {
			return Some(self);
		} else*/
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id_);
            }
        }
        None
    }
    fn is_single_mut(&mut self) -> Option<&mut UiSingleContainer> {
        Some(self)
    }
    fn is_single(&self) -> Option<&UiSingleContainer> {
        Some(self)
    }
    fn as_member(&self) -> &UiMember {
    	self
    }
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}

impl UiMember for Window {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.visibility = visibility;
        if types::Visibility::Visible == visibility {
            self.window.slots().set_visible();
        } else {
            self.window.slots().set_hidden();
        }
    }
    fn visibility(&self) -> types::Visibility {
        self.base.visibility
    }
    fn size(&self) -> (u16, u16) {
        let size = self.window.size();
        (size.width() as u16, size.height() as u16)
    }
    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.h_resize = handler;
        
    }
	unsafe fn native_id(&self) -> usize {
    	self.window.win_id() as usize
    }
    
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
    	None
    }
    fn is_control(&self) -> Option<&UiControl> {
    	None
    }
    fn as_base(&self) -> &types::UiMemberBase {
    	self.base.as_ref()
    }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase {
    	self.base.as_mut()
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.filter.clear();
    }
}

unsafe fn is_control(_: &development::UiMemberCommon) -> Option<&development::UiControlCommon> {
    None
}
unsafe fn is_control_mut(_: &mut development::UiMemberCommon) -> Option<&mut development::UiControlCommon> {
    None
}
impl_size!(Window);
impl_member_id!(MEMBER_ID_WINDOW);

fn event_handler(object: &mut Object, event: &Event) -> bool {
	unsafe {
		if event.type_() == Type::Resize {
			let ptr = object.property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
			if ptr != 0 {
				let window: &mut Window = ::std::mem::transmute(ptr);
				println!("Resize {:?} = {:?}", event.type_(), window.size()); 
			}
			let cls = ::std::ffi::CString::from_raw((&*object.meta_object()).class_name() as *mut i8);
			println!("name {:?}", cls); 
			::std::mem::forget(cls);
		}
		true
	}
}

