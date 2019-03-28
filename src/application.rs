use crate::common::{self, *};
use crate::window::Window;
use crate::tray::Tray;

use qt_widgets::application::Application as QApplication;
use qt_core::core_application::{CoreApplication as QCoreApplication, CoreApplicationArgs as QCoreApplicationArgs};
use qt_core::cpp_utils::CppBox;
use qt_core::string::String;

use plygui_api::development;
use plygui_api::{controls, ids, types};

use std::borrow::Cow;
use std::process::exit;

pub type Application = development::Application<QtApplication>;

pub struct QtApplication {
    _args: QCoreApplicationArgs,
    inner: CppBox<QApplication>,
    pub(crate) windows: Vec<QtId>,
    pub(crate) trays: Vec<QtId>,
}

impl ApplicationInner for QtApplication {
    fn get() -> Box<Application> {
        let mut args = QCoreApplicationArgs::from_real();
        let inner = unsafe { QApplication::new(args.get()) };
        //QCoreApplication::set_application_name(&String::from_std_str(name));
        Box::new(development::Application::with_inner(QtApplication { _args: args, inner: inner, windows: Vec::with_capacity(1), trays: vec![] }, ()))
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        use plygui_api::controls::HasNativeId;

        let w = super::window::QtWindow::with_params(title, size, menu);
        self.windows.push(unsafe { w.native_id().into() });
        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        use plygui_api::controls::HasNativeId;

        let t = super::tray::QtTray::with_params(title, menu);
        self.trays.push(unsafe { t.native_id().into() });
        t
    }
    fn remove_window(&mut self, id: Self::Id) {
        self.windows.retain(|w| *w != id);
    }
    fn remove_tray(&mut self, id: Self::Id) {
        self.trays.retain(|t| *t != id);
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
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Member> {
        use plygui_api::controls::{Container, Member};
        use std::ops::DerefMut;
        
        for window in self.windows.as_mut_slice() {
            let window = common::cast_qobject_to_uimember_mut::<Window>(window.deref_mut()).unwrap();
            if window.id() == id {
                return Some(window);
            } else {
                return window.find_control_by_id_mut(id).map(|control| control.as_member_mut());
            }
        }
        None
    }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&dyn controls::Member> {
        use plygui_api::controls::{Container, Member};

        for window in self.windows.as_slice() {
            let window = common::cast_qobject_to_uimember::<Window>(window).unwrap();
            if window.id() == id {
                return Some(window);
            } else {
                return window.find_control_by_id(id).map(|control| control.as_member());
            }
        }
        None
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        use crate::plygui_api::controls::Closeable;

        let mut n = self.windows.len() as isize;
        let mut i = n - 1;
        while i >= 0 {
            let window = &mut self.windows[i as usize];
            if let Some(window) = common::cast_qobject_to_uimember_mut::<Window>(window) {
                if !window.close(skip_on_close) {
                    return false;
                }
            }
            i -= 1;
        }

        n = self.trays.len() as isize;
        i = n - 1;
        while i >= 0 {
            let tray = &mut self.trays[i as usize];
            if let Some(tray) = common::cast_qobject_to_uimember_mut::<Tray>(tray) {
                if !tray.close(skip_on_close) {
                    return false;
                }
            }
            i -= 1;
        }
        
        QCoreApplication::exit(0);
        true
    }
}
impl HasNativeIdInner for QtApplication {
    type Id = common::QtId;
    
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.inner.static_cast() as *const QObject as *mut QObject)
    }
}
impl Drop for QtApplication {
    fn drop(&mut self) {
        QApplication::close_all_windows();
    }
}
