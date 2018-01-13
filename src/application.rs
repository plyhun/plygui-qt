use super::*;
use super::common::*;

use qt_widgets::application::{Application as QApplication};
use qt_widgets::main_window::{MainWindow as QMainWindow};
use qt_core::string::String;
use qt_core::core_application::{CoreApplication as QCoreApplication, CoreApplicationArgs as QCoreApplicationArgs};
use qt_core::cpp_utils::CppBox;

//use plygui_api::members::MEMBER_ID_APPLICATION;
use plygui_api::traits::{UiWindow, UiApplication, UiMember};
use plygui_api::types::WindowStartSize;
use plygui_api::ids::Id;

use std::borrow::Cow;
use std::process::exit;

pub struct Application {
    inner: CppBox<QApplication>,
    windows: Vec<CppBox<QMainWindow>>,
}

impl Application {
    pub fn with_name(name: &str) -> Box<Application> {
    	let inner = unsafe { 
    		QApplication::new(QCoreApplicationArgs::from_real().get()) 
        };
    	QCoreApplication::set_application_name(&String::from_std_str(name));
        Box::new(
        	Application { 
        		inner: inner,
        		windows: Vec::with_capacity(1),
	        }
        )
    }
}

impl UiApplication for Application {
    fn new_window(&mut self, title: &str, size: WindowStartSize, has_menu: bool) -> Box<UiWindow> {
        let mut w = Window::new(title, size, has_menu);
        self.windows.push(w.qwindow());
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
    fn find_member_by_id_mut(&mut self, id: Id) -> Option<&mut UiMember> {
    	use plygui_api::traits::UiContainer;
    	
    	for window in self.windows.as_mut_slice() {
    		let window = unsafe { common::cast_qobject_mut::<window::Window>(window.as_mut().static_cast_mut() as &mut QObject) };
    		if window.as_base().id() == id {
    			return Some(window);
    		} else {
    			return window.find_control_by_id_mut(id).map(|control| control.as_member_mut());
    		}
    	}
    	None
    }
    fn find_member_by_id(&self, id: Id) -> Option<&UiMember> {
    	use plygui_api::traits::UiContainer;
    	
    	for window in self.windows.as_slice() {
    		let window = unsafe { common::cast_qobject::<window::Window>(window.as_ref().static_cast() as &QObject) };
    		if window.as_base().id() == id {
    			return Some(window);
    		} else {
    			return window.find_control_by_id(id).map(|control| control.as_member());
    		}
    	}
    	None
    }
}

impl Drop for Application {
    fn drop(&mut self) {
    	QApplication::close_all_windows();
    }
}
