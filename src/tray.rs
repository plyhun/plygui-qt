use crate::common::{self, *};

use qt_widgets::QApplication;
use qt_widgets::q_style::StandardPixmap;
use qt_widgets::QSystemTrayIcon;
use qt_gui::QIcon;

use std::borrow::Cow;

pub type Tray = AMember<ATray<QtTray>>;

#[repr(C)]
pub struct QtTray {
    tray: CppBox<QSystemTrayIcon>,
    filter: CppBox<CustomEventFilter>,
    menu: Option<(CppBox<QMenu>, Vec<(callbacks::Action, Slot<'static>)>)>,
    on_close: Option<callbacks::OnClose>,
    skip_callbacks: bool,
}

impl HasLabelInner for QtTray {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = (&*self.tray.as_ref()).tool_tip().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data().as_raw_ptr() as *const u8, name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.tray.set_tool_tip(&QString::from_std_str(label)); }
    }
}

impl CloseableInner for QtTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.skip_callbacks = skip_callbacks;
        if !skip_callbacks {
            if let Some(ref mut on_close) = self.on_close {
                let w2 = common::cast_qobject_to_uimember_mut::<Tray>(unsafe { &mut self.tray.static_upcast_mut::<QObject>() }).unwrap();
                if !(on_close.as_mut())(w2) {
                    return false;
                }
            }
        }
        unsafe { self.tray.hide(); }
        crate::application::Application::get()
            .unwrap()
            .as_any_mut()
            .downcast_mut::<crate::application::Application>()
            .unwrap()
            .inner_mut()
            .remove_tray(unsafe { self.native_id() });
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
}
impl HasImageInner for QtTray {
	fn image(&self, _base: &MemberBase) -> Cow<image::DynamicImage> {
        unimplemented!()
    }
    fn set_image(&mut self, _base: &mut MemberBase, i: Cow<image::DynamicImage>) {
    	use image::GenericImageView;
	    let i = {
    		let status_size = 22; //self.tray.size() as u32;
    		i.resize(status_size, status_size, image::FilterType::Lanczos3)
    	};
    	let mut raw = i.to_rgba().into_raw();
	    let (w, h) = i.dimensions();
        let i = unsafe { QImage::from_uchar2_int_format(MutPtr::from_raw(raw.as_mut_ptr()), w as i32, h as i32, Format::FormatRGBA8888) };
        unsafe { self.tray.set_icon(QIcon::from_q_pixmap(QPixmap::from_image_1a(i.as_ref()).as_ref()).as_ref()); }
    }
}
impl TrayInner for QtTray {
    fn with_params<S: AsRef<str>>(title: S, menu: types::Menu) -> Box<dyn controls::Tray> {
        let icon = unsafe { QApplication::style().standard_icon_1a(StandardPixmap::SPDesktopIcon) };
        let tray = unsafe { QSystemTrayIcon::new() };

        let mut tray = Box::new(AMember::with_inner(
            ATray::with_inner(
                QtTray {
                    tray: tray,
                    filter: CustomEventFilter::new(event_handler),
                    menu: None,
                    on_close: None,
                    skip_callbacks: false,
                }
            ),
        ));
        unsafe {
            let ptr = tray.as_ref() as *const _ as u64;
            (tray.inner_mut().inner_mut().tray.static_upcast_mut::<QObject>()).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        controls::HasLabel::set_label(tray.as_mut(), title.as_ref().into());
        {
            let selfptr = tray.as_mut() as *mut Tray;
            let tray = tray.inner_mut().inner_mut();
            //tray.tray.set_size_policy((QPolicy::Ignored, QPolicy::Ignored));
            //tray.tray.set_minimum_size((1, 1));
            unsafe {
                let filter = tray.filter.static_upcast_mut::<QObject>();
                let mut qobject = tray.tray.static_upcast_mut::<QObject>();
                qobject.install_event_filter(filter);
                tray.tray.set_icon(icon.as_ref());
            }
            
            if let Some(items) = menu {
                tray.menu = Some((unsafe { QMenu::new() }, Vec::new()));
                if let Some((ref mut context_menu, ref mut storage)) = tray.menu {
                    fn slot_spawn(id: usize, selfptr: *mut Tray) -> Slot<'static> {
                        Slot::new(move || {
                            let tray = unsafe { &mut *selfptr };
                            if let Some((_, ref mut menu)) = tray.inner_mut().inner_mut().menu {
                                if let Some((a, _)) = menu.get_mut(id) {
                                    let tray = unsafe { &mut *selfptr };
                                    (a.as_mut())(tray);
                                }
                            }
                        })
                    }

                    common::make_menu(context_menu, items, storage, slot_spawn, selfptr);
                    unsafe {
                        tray.tray.set_context_menu(context_menu);
                    }
                } else {
                    unreachable!();
                }
            }
            unsafe { tray.tray.show(); }
        }
        tray
    }
}

impl HasNativeIdInner for QtTray {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.tray.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl HasVisibilityInner for QtTray {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        unsafe {
            if types::Visibility::Visible == value {
                self.tray.show();
            } else {
                self.tray.hide();
            }
        }
        true
    }
}
impl MemberInner for QtTray {}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Hide => {
            let object2 = object as *mut QObject;
            if let Some(w) = common::cast_qobject_to_uimember_mut::<Tray>(object) {
                if !w.inner().inner().skip_callbacks {
                    if let Some(ref mut on_close) = w.inner_mut().inner_mut().on_close {
                        let w2 = common::cast_qobject_to_uimember_mut::<Tray>(unsafe { &mut *object2 }).unwrap();
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
                    .remove_tray(unsafe { w.inner_mut().native_id() });
            }
        }
        _ => {}
    }
    false
}
