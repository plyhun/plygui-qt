use crate::common::{self, *};

use qt_widgets::application::Application as QApplication;
use qt_widgets::style::StandardPixmap;
use qt_widgets::system_tray_icon::SystemTrayIcon as QSystemTrayIcon;

use std::borrow::Cow;

pub type Tray = Member<QtTray>;

#[repr(C)]
pub struct QtTray {
    tray: CppBox<QSystemTrayIcon>,
    filter: CppBox<CustomEventFilter>,
    on_close: Option<callbacks::Action>,
    skip_callbacks: bool,
}

impl HasLabelInner for QtTray {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let name = (&*self.tray.as_ref()).tool_tip().to_utf8();
        unsafe {
            let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.tray.set_tool_tip(&QString::from_std_str(label));
    }
}

impl CloseableInner for QtTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.skip_callbacks = skip_callbacks;
        self.tray.hide();
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::Action>) {
        self.on_close = callback;
    }
}

impl TrayInner for QtTray {
    fn with_params(title: &str, _menu: types::Menu) -> Box<Member<Self>> {
        use plygui_api::controls::HasLabel;
        
        let icon = unsafe{&mut *QApplication::style()}.standard_icon(StandardPixmap::DesktopIcon);
        let tray = QSystemTrayIcon::new(());

        let mut tray = Box::new(Member::with_inner(
            QtTray {
                tray: tray,
                filter: CustomEventFilter::new(event_handler),
                on_close: None,
                skip_callbacks: false,
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = tray.as_ref() as *const _ as u64;
            (tray.as_inner_mut().tray.as_mut().static_cast_mut() as &mut QObject).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        tray.set_label(title);
        {
            let tray = tray.as_inner_mut();
            //tray.tray.set_size_policy((QPolicy::Ignored, QPolicy::Ignored));
            //tray.tray.set_minimum_size((1, 1));
            unsafe {
                let filter: *mut QObject = tray.filter.static_cast_mut() as *mut QObject;
                let qobject: &mut QObject = tray.tray.as_mut().static_cast_mut();
                qobject.install_event_filter(filter);
            }
            tray.tray.set_icon(&icon);
            tray.tray.show();
        }
        tray
    }
}

impl HasNativeIdInner for QtTray {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.tray.static_cast() as *const QObject as *mut QObject)
    }
}
impl HasVisibilityInner for QtTray {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        if types::Visibility::Visible == value {
            self.tray.slots().show();
        } else {
            self.tray.slots().hide();
        }
        true
    }
}
impl MemberInner for QtTray {
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Hide => {
            let object2 = object as *mut QObject;
            if let Some(w) = common::cast_qobject_to_uimember_mut::<Tray>(object) {
                if !w.as_inner_mut().skip_callbacks {
                    if let Some(ref mut on_close) = w.as_inner_mut().on_close {
                        let w2 = common::cast_qobject_to_uimember_mut::<Tray>(unsafe { &mut *object2 }).unwrap();
                        if !(on_close.as_mut())(w2) {
                            event.ignore();
                            return true;
                        }
                    }
                }
                let mut app = super::application::QtApplication::get();
                app.as_inner_mut().trays.retain(|ww| *ww == unsafe { w.as_inner_mut().native_id() });
            }
            dbg!("hide");
        }
        _ => {}
    }
    false
}

default_impls_as!(Tray);
