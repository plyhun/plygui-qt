use crate::common::{self, *};

use qt_widgets::QApplication;
use qt_widgets::QMainWindow;

use std::borrow::Cow;

pub type Window = AMember<AContainer<ASingleContainer<AWindow<QtWindow>>>>;

#[repr(C)]
pub struct QtWindow {
    window: CppBox<QMainWindow>,
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
}
impl WindowInner for QtWindow {
    fn with_params<S: AsRef<str>>(title: S, start_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let mut window = Box::new(AMember::with_inner(
            AContainer::with_inner(
                ASingleContainer::with_inner(
                    AWindow::with_inner(
                        QtWindow {
                            window: unsafe { QMainWindow::new_0a() },
                            child: None,
                            filter: CustomEventFilter::new(event_handler::<Window>),
                            menu: if menu.is_some() { Vec::new() } else { Vec::with_capacity(0) },
                            on_close: None,
                            skip_callbacks: false,
                        },
                    ),
                ),
            )
        ));
        unsafe {
            let ptr = window.as_ref() as *const _ as u64;
            (window.inner_mut().inner_mut().inner_mut().inner_mut().window.static_upcast_mut::<QObject>()).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        controls::HasLabel::set_label(window.as_mut(), title.as_ref().into());
        {
            let selfptr = window.as_ref() as *const _ as u64;
            let window = window.inner_mut().inner_mut().inner_mut().inner_mut();
            unsafe {
                let (w, h) = match start_size {
                    types::WindowStartSize::Exact(w, h) => (w as i32, h as i32),
                    types::WindowStartSize::Fullscreen => {
                        let screen = QApplication::desktop().screen_geometry();
                        (screen.width(), screen.height())
                    }
                };
                window.window.resize_2a(w, h);
                window.window.set_size_policy_2a(QSizePolicy::Ignored, QSizePolicy::Ignored);
                window.window.set_minimum_size_2a(1, 1);
                let filter = window.filter.static_upcast_mut::<QObject>();
                let mut qobject = window.window.static_upcast_mut::<QObject>();
                qobject.install_event_filter(filter);
            }
            if let Some(mut items) = menu {
                let mut menu_bar = unsafe { window.window.menu_bar() };

                fn slot_spawn(id: usize, selfptr: *mut Window) -> Slot<'static> {
                    Slot::new(move || {
                        let window = unsafe { &mut *selfptr };
                        if let Some((a, _)) = window.inner_mut().inner_mut().inner_mut().inner_mut().menu.get_mut(id) {
                            let window = unsafe { &mut *selfptr };
                            (a.as_mut())(window);
                        }
                    })
                }
                let mut app = crate::application::Application::get().unwrap();
                let app = app
                    .as_any_mut()
                    .downcast_mut::<crate::application::Application>()
                    .unwrap()
                    .inner_mut();

                for item in items.drain(..) {
                    match item {
                        types::MenuItem::Action(label, action, _) => {
                            let id = window.menu.len();
                            let action = (action, slot_spawn(id, selfptr as *mut Window));
                            unsafe { 
                                let qaction = menu_bar.add_action_1a(QString::from_std_str(label).as_ref());
                                qaction.triggered().connect(&app.queue);
                            }
                            window.menu.push(action);
                        }
                        types::MenuItem::Sub(label, items, _) => {
                            let mut submenu = unsafe { menu_bar.add_menu_q_string(&QString::from_std_str(label)) };
                            common::make_menu(&mut submenu, items, &mut window.menu, slot_spawn, selfptr as *mut Window);
                        }
                        types::MenuItem::Delimiter => {
                            unsafe { menu_bar.add_separator(); }
                        }
                    }
                }
            }
            unsafe { window.window.show(); }
        }
        window
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

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.window.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
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
                window.call_on_size(width, height);
            }
        }
        QEventType::Close => {
            let object2 = object as *mut QObject;
            if let Some(w) = common::cast_qobject_to_uimember_mut::<Window>(object) {
                if !w.inner_mut().inner_mut().inner_mut().inner_mut().skip_callbacks {
                    if let Some(ref mut on_close) = w.inner_mut().inner_mut().inner_mut().inner_mut().on_close {
                        let w2 = common::cast_qobject_to_uimember_mut::<O>(unsafe { &mut *object2 }).unwrap();
                        if !(on_close.as_mut())(w2) {
                            unsafe { event.ignore(); }
                            return true;
                        }
                    }
                }
                crate::application::Application::get()
                    .unwrap()
                    .as_any_mut()
                    .downcast_mut::<crate::application::Application>()
                    .unwrap()
                    .inner_mut()
                    .remove_window(unsafe { w.inner_mut().inner_mut().inner_mut().native_id() });
            }
        }
        _ => {}
    }
    false
}
