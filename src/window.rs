use crate::common::{self, *};

use qt_widgets::QApplication;
use qt_widgets::QMainWindow;

use std::borrow::Cow;

pub type Window = AMember<AContainer<ASingleContainer<ACloseable<AWindow<QtWindow>>>>>;

#[repr(C)]
pub struct QtWindow {
    window: common::MaybeCppBox<QMainWindow>,
    child: Option<Box<dyn controls::Control>>,
    filter: CppBox<CustomEventFilter>,
    menu: Vec<(callbacks::Action, Slot<'static>)>,
    on_close: Option<callbacks::OnClose>,
    skip_callbacks: bool,
}

impl HasLabelInner for QtWindow {
    fn label<'a>(&'a self, _: &MemberBase) -> Cow<'a, str> {
        unsafe {
            let name = self.window.as_ref().window_title().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data().as_raw_ptr() as *const u8, name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.window.set_window_title(&QString::from_std_str(&label)); }
    }
}

impl CloseableInner for QtWindow {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.skip_callbacks = skip_callbacks;
        unsafe { self.window.close() }
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
    fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn controls::Application {
        unsafe { utils::base_to_impl::<Window>(base) }.inner().inner().inner().application_impl::<crate::application::Application>()
    }
    fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn controls::Application {
        unsafe { utils::base_to_impl_mut::<Window>(base) }.inner_mut().inner_mut().inner_mut().application_impl_mut::<crate::application::Application>()
    }
}
impl<O: controls::Window> NewWindowInner<O> for QtWindow {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, app: &mut dyn controls::Application, title: &str, start_size: types::WindowStartSize, menu: types::Menu) -> Self {
   		let selfptr = u as *mut _ as *mut Window;
   		let mut w = QtWindow {
            window: common::MaybeCppBox::Some(unsafe { QMainWindow::new_0a() }),
            child: None,
            filter: CustomEventFilter::new(event_handler::<Window>),
            menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
            on_close: None,
            skip_callbacks: false,
        };
   		unsafe {
            w.window.static_upcast_mut::<QObject>().set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(selfptr as u64));
            w.window.set_window_title(&QString::from_std_str(&title));
            let (ww, hh) = match start_size {
                types::WindowStartSize::Exact(w, h) => (w as i32, h as i32),
                types::WindowStartSize::Fullscreen => {
                    let screen = QApplication::desktop().screen_geometry();
                    (screen.width(), screen.height())
                }
            };
            w.window.resize_2a(ww, hh);
            w.window.set_size_policy_2a(QSizePolicy::Ignored, QSizePolicy::Ignored);
            w.window.set_minimum_size_2a(1, 1);
            let filter = w.filter.static_upcast_mut::<QObject>();
            let mut qobject = w.window.static_upcast_mut::<QObject>();
            qobject.install_event_filter(filter);
        }
        if let Some(mut items) = menu {
            let mut menu_bar = unsafe { w.window.menu_bar() };

            fn slot_spawn(id: usize, selfptr: *mut Window) -> Slot<'static> {
                Slot::new(move || {
                    let window = unsafe { &mut *selfptr };
                    if let Some((a, _)) = window.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().menu.get_mut(id) {
                        let window = unsafe { &mut *selfptr };
                        (a.as_mut())(window);
                    }
                })
            }
            for item in items.drain(..) {
                match item {
                    types::MenuItem::Action(label, action, _) => {
                        let id = w.menu.len();
                        let action = (action, slot_spawn(id, selfptr));
                        unsafe { 
                            let qaction = menu_bar.add_action_1a(QString::from_std_str(label).as_ref());
                            qaction.triggered().connect(&app.as_any().downcast_ref::<crate::application::Application>().unwrap().inner().queue);
                        }
                        w.menu.push(action);
                    }
                    types::MenuItem::Sub(label, items, _) => {
                        let mut submenu = unsafe { menu_bar.add_menu_q_string(&QString::from_std_str(label)) };
                        common::make_menu(&mut submenu, items, &mut w.menu, slot_spawn, selfptr as *mut Window);
                    }
                    types::MenuItem::Delimiter => {
                        unsafe { menu_bar.add_separator(); }
                    }
                }
            }
        }
        w
    }
}
impl WindowInner for QtWindow {
    fn with_params<S: AsRef<str>>(app: &mut dyn controls::Application, title: S, start_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let app = app.as_any_mut().downcast_mut::<crate::application::Application>().unwrap();
        let mut b: Box<mem::MaybeUninit<Window>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AContainer::with_inner(
	            ASingleContainer::with_inner(
	                ACloseable::with_inner(
    	                AWindow::with_inner(
    	                    <Self as NewWindowInner<Window>>::with_uninit_params(b.as_mut(), app, title.as_ref(), start_size, menu),
    	                ),
    	                app
	                )
	            )
            ),
        );
        let mut w = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        unsafe { w.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().window.show(); }
        w
    }
    fn size(&self) -> (u16, u16) {
        unsafe {
            let size = self.window.size();
            (size.width() as u16, size.height() as u16)
        }
    }
    fn position(&self) -> (i32, i32) {
        unsafe {
            let pos = self.window.pos();
            (pos.x() as i32, pos.y() as i32)
        }
    }
}

impl SingleContainerInner for QtWindow {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        let (w, h) = self.size();
        let margins = unsafe { self.window.contents_margins() };
        if let Some(old) = old.as_mut() {
            old.on_removed_from_container(unsafe { utils::base_to_impl_mut::<Window>(base) });
        }
        if let Some(new) = child.as_mut() {
            unsafe {
                let widget = common::cast_control_to_qwidget_mut(new.as_mut());
                self.window.set_central_widget(MutPtr::from_raw(widget));
                new.on_added_to_container(
                    utils::base_to_impl_mut::<Window>(base),
                    0,
                    0,
                    utils::coord_to_size(cmp::max(0, w as i32 - margins.left() - margins.right())),
                    utils::coord_to_size(cmp::max(0, h as i32 - margins.top() - margins.bottom())),
                );
            }
        } else {
            unsafe {
                self.window.set_central_widget(QWidget::new_0a().as_mut_ptr());
            }
        }
        self.child = child;

        old
    }
    fn child(&self) -> Option<&dyn controls::Control> {
        self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl ContainerInner for QtWindow {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_mut(arg);
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control(arg);
            }
        }
        None
    }
}
impl HasNativeIdInner for QtWindow {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.window.static_upcast::<QObject>() }.as_raw_ptr() as *mut QObject)
    }
}
impl HasSizeInner for QtWindow {
    fn on_size_set(&mut self, _: &mut MemberBase, (w, h): (u16, u16)) -> bool {
        unsafe { self.window.set_fixed_size_2a(w as i32, h as i32) };
        true
    }
}
impl HasVisibilityInner for QtWindow {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        unsafe { self.window.set_visible(types::Visibility::Visible == value) };
        true
    }
}
impl MemberInner for QtWindow {}
impl Drop for QtWindow {
    fn drop(&mut self) {
        self.filter.clear();
    }
}

fn event_handler<O: controls::Window>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(window) = common::cast_qobject_to_uimember_mut::<Window>(object) {
                let (width, height) = window.inner().inner().inner().size();
                window.call_on_size::<O>(width, height);
                if let Some(ref mut child) = window.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().child {
                    child.measure(width as u16, height as u16);
                }
            }
        }
        QEventType::Close => {
            let object2 = object as *mut QObject;
            if let Some(w) = common::cast_qobject_to_uimember_mut::<Window>(object) {
                use crate::plygui_api::controls::Member;
                
                if !w.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().skip_callbacks {
                    if let Some(ref mut on_close) = w.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().on_close {
                        let w2 = common::cast_qobject_to_uimember_mut::<O>(unsafe { &mut *object2 }).unwrap();
                        if !(on_close.as_mut())(w2) {
                            unsafe { event.ignore(); }
                            return true;
                        }
                    }
                }
                let id = w.id();
                let app = w.inner_mut().inner_mut().inner_mut().application_impl_mut::<crate::application::Application>();
                let _ = app.base.sender().send((move |a: &mut dyn controls::Application| {
                    a.as_any_mut().downcast_mut::<crate::application::Application>().unwrap().base.windows.retain(|w| w.id() != id);
                    false
                }).into());
            }
        }
        QEventType::Destroy => {
            if let Some(w) = cast_qobject_to_uimember_mut::<Window>(object) {
                unsafe {
                    ptr::write(&mut w.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().window, common::MaybeCppBox::None);
                }
            }
        }
        _ => {}
    }
    false
}
