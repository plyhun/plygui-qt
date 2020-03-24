use crate::common::{self, *};

use qt_core::{QCoreApplication, QCoreApplicationArgs};
use qt_core::QTimer;
use qt_core::QString;
use qt_core::ConnectionType;
use qt_gui::QGuiApplication;
use qt_widgets::QApplication;

use plygui_api::{controls, types};

use std::borrow::Cow;
use std::process::exit;
use std::any::TypeId;

const DEFAULT_FRAME_SLEEP_MS: u32 = 10;

pub type Application = AApplication<QtApplication>;

pub struct QtApplication {
    _args: QCoreApplicationArgs,
    inner: QBox<QApplication>,
    timer: QBox<QTimer>,
    pub(crate) queue: QBox<SlotNoArgs>,
}

impl QtApplication {
    pub fn maybe_exit(&mut self) -> bool {
        let base = &mut unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().base;
        if base.windows.len() < 1 && base.trays.len() < 1 {
            unsafe { QCoreApplication::quit(); }
            true
        } else {
            false
        }
    }
}

impl<O: controls::Application> NewApplicationInner<O> for QtApplication {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, name: &str) -> Self {
        let mut args = QCoreApplicationArgs::new();
        let (arg1, arg2) = args.get();
        let inner = unsafe { QApplication::new_2a(arg1, arg2) };
        unsafe { 
            QGuiApplication::set_quit_on_last_window_closed(false);
            QCoreApplication::set_application_name(&QString::from_std_str(name));
        }
        let mut a = QtApplication {
            _args: args,
            inner: inner,
            timer: unsafe { QTimer::new_0a() },
            queue: unsafe { SlotNoArgs::new(NullPtr, move || {}) },
        };
        let selfptr = u as *const _ as u64;
        unsafe {
            a.inner.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(selfptr));
        }
        a.set_frame_sleep(DEFAULT_FRAME_SLEEP_MS);
        let handler = move || {
            let mut frame_callbacks = 0;
            while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                let w = &mut unsafe { &mut *(selfptr as *mut Application) }.base;
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
        };
        a.queue = unsafe { SlotNoArgs::new(NullPtr, handler) };
        unsafe { a.timer.timeout().connect_with_type(ConnectionType::QueuedConnection, &a.queue) };
        a
    }
}

impl ApplicationInner for QtApplication {
    fn with_name<S: AsRef<str>>(name: S) -> Box<dyn controls::Application> {
        let mut b: Box<mem::MaybeUninit<Application>> = Box::new_uninit();
        let ab = AApplication::with_inner(
            <Self as NewApplicationInner<Application>>::with_uninit_params(b.as_mut(), name.as_ref()),
        );
        let mut a = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        unsafe {
        	a.inner_mut().timer.start_0a();
        }
        a
    }
	fn frame_sleep(&self) -> u32 {
		let interval = unsafe { self.timer.interval() };
		if interval > -1 { interval as u32 } else { 0 }
	}
	fn set_frame_sleep(&mut self, value: u32) {
		unsafe { self.timer.set_interval(value as i32) };
	}
    fn name<'a>(&'a self) -> Cow<'a, str> {
        unsafe {
            let name = QCoreApplication::application_name().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data(), name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(mem::transmute(bytes)).to_owned())
        }
    }
    fn start(&mut self) {
        exit(unsafe { QApplication::exec() });
    }
    fn exit(&mut self) {
        let base = &mut unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().base;
        for mut window in base.windows.drain(..) {
            window.as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(true);
        }
        for mut tray in base.trays.drain(..) {
            tray.as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(true);
        }
        self.maybe_exit();
    }
    fn add_root(&mut self, m: Box<dyn controls::Closeable>) -> &mut dyn controls::Member {
        let base = &mut unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().base;
        
        let is_window = m.as_any().type_id() == TypeId::of::<crate::window::Window>();
        let is_tray = m.as_any().type_id() == TypeId::of::<crate::tray::Tray>();
        
        if is_window {
            let i = base.windows.len();
            base.windows.push(m.into_any().downcast::<crate::window::Window>().unwrap());
            return base.windows[i].as_mut().as_member_mut();
        }
        
        if is_tray {
            let i = base.trays.len();
            base.trays.push(m.into_any().downcast::<crate::tray::Tray>().unwrap());
            return base.trays[i].as_mut().as_member_mut();
        }
        
        panic!("Unsupported Closeable: {:?}", m.as_any().type_id());
    }
    fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool {
        let base = &mut unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().base;
        match arg {
            types::FindBy::Id(id) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].id() == id 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].id() == id 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
            }
            types::FindBy::Tag(ref tag) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].tag().is_some() && base.windows[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].tag().is_some() && base.trays[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        self.maybe_exit();
                        true
                    } else {
                        false
                }).is_some()
            }
        }
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        let base = &mut unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().base;
        for window in base.windows.as_mut_slice() {
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
            let found = controls::Container::find_control_mut(window.as_mut(), arg.clone()).map(|control| control.as_member_mut());
            if found.is_some() {
                return found;
            }
        }
        for tray in base.trays.as_mut_slice() {
            let tray = &mut **tray;
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
        None
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        let base = & unsafe { common::cast_qobject::<Application>(&self.inner) }.unwrap().base;
        for window in base.windows.as_slice() {
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
            let found = controls::Container::find_control(window.as_ref(), arg.clone()).map(|control| control.as_member());
            if found.is_some() {
                return found;
            }
        }
        for tray in base.trays.as_slice() {
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
        None
    }
    fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        unsafe { common::cast_qobject::<Application>(&self.inner) }.unwrap().roots()
    }
    fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        unsafe { common::cast_qobject_mut::<Application>(&mut self.inner) }.unwrap().roots_mut()
    }
}
impl HasNativeIdInner for QtApplication {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.inner.static_upcast::<QObject>().as_raw_ptr() } as *const QObject as *mut QObject)
    }
}
impl Drop for QtApplication {
    fn drop(&mut self) {
        unsafe { QApplication::close_all_windows() };
    }
}
