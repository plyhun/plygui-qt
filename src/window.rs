use super::common::*;
use super::*;

use qt_widgets::application::Application as QApplication;
use qt_widgets::main_window::MainWindow as QMainWindow;
use qt_core::timer::Timer;
use qt_core::slots::SlotNoArgs;

use std::borrow::Cow;

pub type Window = Member<SingleContainer<::plygui_api::development::Window<QtWindow>>>;

#[repr(C)]
pub struct QtWindow {
    window: CppBox<QMainWindow>,
    child: Option<Box<controls::Control>>,
    filter: CppBox<CustomEventFilter>,
    timer: CppBox<Timer>,
    queue: SlotNoArgs<'static>,
}

impl HasLabelInner for QtWindow {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let name = (&*self.window.as_ref()).window_title().to_utf8();
        unsafe {
            let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.window.set_window_title(&QString::from_std_str(label));
    }
}

impl WindowInner for QtWindow {
    fn with_params(title: &str, start_size: types::WindowStartSize, _menu: types::WindowMenu) -> Box<Member<SingleContainer<::plygui_api::development::Window<Self>>>> {
        use plygui_api::controls::HasLabel;

        let mut window = Box::new(Member::with_inner(
            SingleContainer::with_inner(
                ::plygui_api::development::Window::with_inner(
                    QtWindow {
                        window: QMainWindow::new(),
                        child: None,
                        filter: CustomEventFilter::new(event_handler),
                        timer: Timer::new(),
                        queue: SlotNoArgs::new(move || {}),
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = window.as_ref() as *const _ as u64;
            (window.as_inner_mut().as_inner_mut().as_inner_mut().window.as_mut().static_cast_mut() as &mut QObject).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        window.set_label(title);
        {
            let selfptr = window.as_ref() as *const _ as u64;
            let window = window.as_inner_mut().as_inner_mut().as_inner_mut();
            window.window.resize(match start_size {
                types::WindowStartSize::Exact(w, h) => (w as i32, h as i32),
                types::WindowStartSize::Fullscreen => {
                    let screen = unsafe { (*QApplication::desktop()).screen_geometry(()) };
                    (screen.width(), screen.height())
                }
            });
            window.window.set_size_policy((QPolicy::Ignored, QPolicy::Ignored));
            window.window.set_minimum_size((1, 1));
            unsafe {
                let filter: *mut QObject = window.filter.static_cast_mut() as *mut QObject;
                let qobject: &mut QObject = window.window.as_mut().static_cast_mut();
                qobject.install_event_filter(filter);
            }
            window.queue = SlotNoArgs::new(move || {
                let mut frame_callbacks = 0;
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    let w = unsafe { (&mut *(selfptr as *mut Window)).as_inner_mut().as_inner_mut().base_mut() };
                    match w.queue().try_recv() {
                        Ok(mut cmd) => {
                            if (cmd.as_mut())(unsafe { &mut *(selfptr as *mut Window) }) {
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
            window.timer.signals().timeout().connect(&window.queue);
            window.timer.start(());
            window.window.show();
        }
        window
    }
    fn on_frame(&mut self, cb: callbacks::Frame) {
        let qobject: &mut QObject = self.window.as_mut().static_cast_mut();
        let _ = cast_qobject_to_uimember_mut::<Window>(qobject).unwrap().as_inner_mut().as_inner_mut().base_mut().sender().send(cb);
    }
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::Frame> {
        let qobject: &mut QObject = self.window.as_mut().static_cast_mut();
        cast_qobject_to_uimember_mut::<Window>(qobject).unwrap().as_inner_mut().as_inner_mut().base_mut().sender().clone().into()
    }
}

impl SingleContainerInner for QtWindow {
    fn set_child(&mut self, base: &mut MemberBase, mut child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
        let mut old = self.child.take();
        let (w, h) = self.size();
        let margins = self.window.contents_margins();
        if let Some(old) = old.as_mut() {
            old.on_removed_from_container(unsafe { utils::base_to_impl_mut::<Window>(base) });
        }
        if let Some(new) = child.as_mut() {
            unsafe {
                let mut widget = common::cast_control_to_qwidget_mut(new.as_mut());
                self.window.as_mut().set_central_widget(widget);
            }
            new.on_added_to_container(
                unsafe { utils::base_to_impl_mut::<Window>(base) },
                0,
                0,
                utils::coord_to_size(cmp::max(0, w as i32 - margins.left() - margins.right())),
                utils::coord_to_size(cmp::max(0, h as i32 - margins.top() - margins.bottom())),
            );
        } else {
            unsafe {
                self.window.as_mut().set_central_widget(QWidget::new().into_raw());
            }
        }
        self.child = child;

        old
    }
    fn child(&self) -> Option<&controls::Control> {
        self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut controls::Control> {
        if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl ContainerInner for QtWindow {
    fn find_control_by_id_mut(&mut self, id_: ids::Id) -> Option<&mut controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id_);
            }
        }
        None
    }
    fn find_control_by_id(&self, id_: ids::Id) -> Option<&controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id_);
            }
        }
        None
    }
}

impl MemberInner for QtWindow {
    type Id = common::QtId;

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        if types::Visibility::Visible == base.visibility {
            self.window.slots().set_visible();
        } else {
            self.window.slots().set_hidden();
        }
    }

    fn size(&self) -> (u16, u16) {
        let size = self.window.size();
        (size.width() as u16, size.height() as u16)
    }

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.window.static_cast() as *const QWidget as *mut QWidget)
    }
}

impl Drop for QtWindow {
    fn drop(&mut self) {
        self.filter.clear();
    }
}

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(window) = common::cast_qobject_to_uimember_mut::<Window>(object) {
                let (width, height) = window.as_inner().as_inner().size();
                if let Some(ref mut child) = window.as_inner_mut().as_inner_mut().as_inner_mut().child {
                    child.measure(width, height);
                    child.draw(Some((0, 0)));
                }
                window.call_on_resize(width, height);
            }
        }
        _ => {}
    }
    false
}

impl_all_defaults!(Window);
