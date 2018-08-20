use super::*;
use super::common::*;

use qt_widgets::application::{Application as QApplication};
//use qt_widgets::main_window::{MainWindow as QMainWindow};
use qt_core::string::String;
use qt_core::core_application::{CoreApplication as QCoreApplication, CoreApplicationArgs as QCoreApplicationArgs};
use qt_core::cpp_utils::CppBox;

use plygui_api::{types, controls, ids};
use plygui_api::development;

use std::borrow::Cow;
use std::process::exit;

pub type Application = development::Application<QtApplication>;

pub struct QtApplication {
    inner: CppBox<QApplication>,
    windows: Vec<QtId>,
}

impl development::ApplicationInner for QtApplication {
    fn with_name(name: &str) -> Box<Application> {
        use plygui_api::development::HasInner;
        
    	let inner = unsafe { 
    		QApplication::new(QCoreApplicationArgs::from_real().get()) 
        };
    	QCoreApplication::set_application_name(&String::from_std_str(name));
        Box::new(development::Application::with_inner(QtApplication { 
        		inner: inner,
        		windows: Vec::with_capacity(1),
	        }, ())
        	
        )
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
        let w = Window::with_params(title, size, menu);
        self.windows.push(unsafe { w.native_id().into() });
        w
    }
    fn name<'a>(&'a self) -> Cow<'a, str> {
        let name = QCoreApplication::application_name().to_utf8();
        unsafe {
	      let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
	      Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
	    }
    }
    fn start(&mut self) {
        exit(QApplication::exec());
    }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Member> {
    	use plygui_api::controls::{Member, Container};
    	
    	for window in self.windows.as_mut_slice() {
    		let window = common::cast_qobject_to_uimember_mut::<window::Window>(window.static_cast_mut() as &mut QObject).unwrap();
    		if window.id() == id {
    			return Some(window);
    		} else {
    			return window.find_control_by_id_mut(id).map(|control| control.as_member_mut());
    		}
    	}
    	None
    }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&controls::Member> {
    	use plygui_api::controls::{Member, Container};
    	
    	for window in self.windows.as_slice() {
    		let window = common::cast_qobject_to_uimember::<window::Window>(window.static_cast() as &QObject).unwrap();
    		if window.id() == id {
    			return Some(window);
    		} else {
    			return window.find_control_by_id(id).map(|control| control.as_member());
    		}
    	}
    	None
    }
}

impl Drop for QtApplication {
    fn drop(&mut self) {
    	QApplication::close_all_windows();
    }
}
