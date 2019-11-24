use crate::common::{self, *};
use crate::tray::Tray;
use crate::window::Window;

use qt_core::core_application::{CoreApplication as QCoreApplication, CoreApplicationArgs as QCoreApplicationArgs};
use qt_core::cpp_utils::CppBox;
use qt_core::timer::Timer;
use qt_core::string::String;
use qt_gui::gui_application::GuiApplication as QGuiApplication;
use qt_widgets::application::Application as QApplication;

use plygui_api::development;
use plygui_api::{controls, types};

use std::borrow::Cow;
use std::process::exit;

pub type Application = development::Application<QtApplication>;

pub struct QtApplication {
    _args: QCoreApplicationArgs,
    inner: CppBox<QApplication>,
    timer: CppBox<Timer>,
    pub(crate) queue: SlotNoArgs<'static>,
    pub(crate) windows: Vec<QtId>,
    pub(crate) trays: Vec<QtId>,
}

impl QtApplication {
    fn maybe_exit(&mut self) -> bool {
        if self.windows.len() < 1 && self.trays.len() < 1 {
            QCoreApplication::exit(0);
            true
        } else {
            false
        }
    }
}

impl ApplicationInner for QtApplication {
	fn frame_sleep(&self) -> u32 {
		let interval = self.timer.interval();
		if interval > -1 { interval as u32 } else { 0 }
	}
	fn set_frame_sleep(&mut self, value: u32) {
		self.timer.set_interval(value as i32);
	}
    fn get() -> Box<Application> {
        let mut args = QCoreApplicationArgs::from_real();
        let inner = unsafe { QApplication::new(args.get()) };
        QGuiApplication::set_quit_on_last_window_closed(false);
        //QCoreApplication::set_application_name(&String::from_std_str(name));
        let mut app = Box::new(development::Application::with_inner(
            QtApplication {
                _args: args,
                inner: inner,
                windows: Vec::with_capacity(1),
                timer: Timer::new(),
                queue: SlotNoArgs::new(move || {}),
                trays: vec![],
            },
            (),
        ));
        {
            let selfptr = app.as_ref() as *const _ as u64;
            let app = app.as_inner_mut();
            app.queue = SlotNoArgs::new(move || {
                let mut frame_callbacks = 0;
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    let w = unsafe { (&mut *(selfptr as *mut Application)).base_mut() };
                    match w.queue().try_recv() {
                        Ok(mut cmd) => {
                            if (cmd.as_mut())(unsafe { &mut *(selfptr as *mut Application) }) {
                                let _ = w.sender().send(cmd);
                            }
                            frame_callbacks += 1;
                        }
                        Err(e) => match e {
                            mpsc::TryRecvError::Empty => break,
                            mpsc::TryRecvError::Disconnected => unreachable!(),
                        },
                    }
                }
            });
            app.timer.signals().timeout().connect(&app.queue);
            app.timer.start(());
        }
        app
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
        self.maybe_exit();
    }
    fn remove_tray(&mut self, id: Self::Id) {
        self.trays.retain(|t| *t != id);
        self.maybe_exit();
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
        self.maybe_exit()
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        use crate::plygui_api::controls::{Member, Container};

        for window in self.windows.as_mut_slice() {
            if let Some(window) = common::cast_qobject_to_uimember_mut::<Window>(window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member_mut());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member_mut());
                            }
                        }
                    }
                }
                let found = window.find_control_mut(arg.clone()).map(|control| control.as_member_mut());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_mut_slice() {
            if let Some(tray) = common::cast_qobject_to_uimember_mut::<Tray>(tray) {
                match arg {
                    types::FindBy::Id(ref id) => {
                        if tray.id() == *id {
                            return Some(tray.as_member_mut());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = tray.tag() {
                            if tag.as_str() == mytag {
                                return Some(tray.as_member_mut());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        use crate::plygui_api::controls::{Member, Container};

        for window in self.windows.as_slice() {
            if let Some(window) = common::cast_qobject_to_uimember::<Window>(window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member());
                            }
                        }
                    }
                }
                let found = window.find_control(arg.clone()).map(|control| control.as_member());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_slice() {
            if let Some(tray) = common::cast_qobject_to_uimember::<Tray>(tray) {
                match arg {
                    types::FindBy::Id(ref id) => {
                        if tray.id() == *id {
                            return Some(tray.as_member());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = tray.tag() {
                            if tag.as_str() == mytag {
                                return Some(tray.as_member());
                            }
                        }
                    }
                }
            }
        }
        None
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        Box::new(MemberIterator {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        Box::new(MemberIteratorMut {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
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
struct MemberIterator<'a> {
    inner: &'a QtApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIterator<'a> {
    type Item = &'a (dyn controls::Member + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get(self.index).map(|tray| common::cast_qobject_to_uimember::<Tray>(tray).unwrap() as &dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get(self.index).map(|window| common::cast_qobject_to_uimember::<Window>(window).unwrap() as &dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}

struct MemberIteratorMut<'a> {
    inner: &'a mut QtApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIteratorMut<'a> {
    type Item = &'a mut (dyn controls::Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_tray && self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get_mut(self.index).map(|tray| common::cast_qobject_to_uimember_mut::<Tray>(tray).unwrap() as &mut dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get_mut(self.index).map(|window| common::cast_qobject_to_uimember_mut::<Window>(window).unwrap() as &mut dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}
