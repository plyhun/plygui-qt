use crate::common::{self, *};

use qt_widgets::QApplication;
use qt_widgets::q_style::StandardPixmap;
use qt_widgets::QSystemTrayIcon;
use qt_gui::QIcon;

use std::borrow::Cow;

pub type Tray = AMember<ACloseable<ATray<QtTray>>>;

#[repr(C)]
pub struct QtTray {
    tray: QBox<QSystemTrayIcon>,
    filter: QBox<CustomEventFilter>,
    menu: Option<(QBox<QMenu>, Vec<(callbacks::Action, QBox<SlotNoArgs>)>)>,
    on_close: Option<callbacks::OnClose>,
    skip_callbacks: bool,
}
impl QtTray {
    fn set_image_inner(&mut self, i: Cow<image::DynamicImage>) {
    	use image::GenericImageView;
	    let i = {
    		let status_size = 22; //self.tray.size() as u32; // TODO
    		i.resize(status_size, status_size, image::imageops::FilterType::Lanczos3)
    	};
    	let mut raw = i.to_rgba8().into_raw();
	    let (w, h) = i.dimensions();
        let i = unsafe { QImage::from_uchar2_int_format(raw.as_mut_ptr(), w as i32, h as i32, Format::FormatRGBA8888) };
        unsafe { self.tray.set_icon(QIcon::from_q_pixmap(QPixmap::from_image_1a(i.as_ref()).as_ref()).as_ref()); }
    }
}
impl HasLabelInner for QtTray {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = self.tray.tool_tip().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data(), name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(mem::transmute(bytes)).to_owned())
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
                let w2 = common::cast_qobject_to_uimember_mut::<Tray>(unsafe { &mut self.tray.static_upcast::<QObject>() }).unwrap();
                if !(on_close.as_mut())(w2) {
                    return false;
                }
            }
        }
        unsafe { self.tray.hide(); }
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
    fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn controls::Application {
        unsafe { utils::base_to_impl::<Tray>(base) }.inner().application_impl::<crate::application::Application>()
    }
    fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn controls::Application {
        unsafe { utils::base_to_impl_mut::<Tray>(base) }.inner_mut().application_impl_mut::<crate::application::Application>()
    }
}
impl HasImageInner for QtTray {
	fn image(&self, _base: &MemberBase) -> Cow<image::DynamicImage> {
        unimplemented!()
    }
    fn set_image(&mut self, _base: &mut MemberBase, i: Cow<image::DynamicImage>) {
    	self.set_image_inner(i)
    }
}
impl<O: controls::Tray> NewTrayInner<O> for QtTray {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, _: &mut dyn controls::Application, title: &str, icon: image::DynamicImage, menu: types::Menu) -> Self {
        let selfptr = u as *mut _ as *mut Tray;
        let qicon = unsafe { QApplication::style().standard_icon_1a(StandardPixmap::SPDesktopIcon) };
        let tray = unsafe { QSystemTrayIcon::new() };
        let mut t = QtTray {
            tray: tray,
            filter: CustomEventFilter::new(event_handler),
            menu: None,
            on_close: None,
            skip_callbacks: false,
        };
        unsafe { 
            t.tray.set_tool_tip(&QString::from_std_str(title)); 
            (t.tray.static_upcast::<QObject>()).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(selfptr as u64));
            let filter = t.filter.static_upcast::<QObject>();
            let qobject = t.tray.static_upcast::<QObject>();
            qobject.install_event_filter(filter);
            t.tray.set_icon(qicon.as_ref());
        }
        {
            if let Some(items) = menu {
                t.menu = Some((unsafe { QMenu::new() }, Vec::new()));
                if let Some((ref mut context_menu, ref mut storage)) = t.menu {
                    fn slot_spawn(id: usize, selfptr: *mut Tray) -> QBox<SlotNoArgs> {
                    	let handler = move || {
                            let t = unsafe { &mut *selfptr };
                            if let Some((_, ref mut menu)) = t.inner_mut().inner_mut().inner_mut().menu {
                                if let Some((a, _)) = menu.get_mut(id) {
                                    let tray = unsafe { &mut *selfptr };
                                    (a.as_mut())(tray);
                                }
                            }
                        };
                        unsafe { SlotNoArgs::new(NullPtr, handler) }
                    }

                    common::make_menu(context_menu, items, storage, slot_spawn, selfptr);
                    unsafe {
                        t.tray.set_context_menu(context_menu.as_ptr());
                    }
                } else {
                    unreachable!();
                }
            }
            unsafe { t.tray.show(); }
        }
        t.set_image_inner(Cow::Owned(icon));
        t
    }
}
impl TrayInner for QtTray {
    fn with_params<S: AsRef<str>>(app: &mut dyn controls::Application, title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn controls::Tray> {
        let mut b: Box<mem::MaybeUninit<Tray>> = Box::new_uninit();
        let ab = AMember::with_inner(
            ACloseable::with_inner(
                ATray::with_inner(
                    <Self as NewTrayInner<Tray>>::with_uninit_params(b.as_mut(), app, title.as_ref(), icon, menu),
                ),
	            app.as_any_mut().downcast_mut::<crate::application::Application>().unwrap()
            )
        );
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasNativeIdInner for QtTray {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.tray.static_upcast::<QObject>().as_raw_ptr() } as *mut QObject)
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
                use crate::plygui_api::controls::Member;
                
                if !w.inner().inner().inner().skip_callbacks {
                    if let Some(ref mut on_close) = w.inner_mut().inner_mut().inner_mut().on_close {
                        let w2 = common::cast_qobject_to_uimember_mut::<Tray>(unsafe { &mut *object2 }).unwrap();
                        if !(on_close.as_mut())(w2) {
                            unsafe { event.ignore(); }
                            return true;
                        }
                    }
                }
                let id = w.id();
                let app = w.inner_mut().application_impl_mut::<crate::application::Application>();
                let _ = app.base.sender().send((move |a: &mut dyn controls::Application| {
                    a.as_any_mut().downcast_mut::<crate::application::Application>().unwrap().base.windows.retain(|w| w.id() != id);
                    false
                }).into());
            }
        }
        _ => {}
    }
    false
}
