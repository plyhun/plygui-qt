pub use qt_core::Signal;
pub use qt_core::QByteArray;
pub use qt_core::{QEvent, q_event::Type as QEventType};
pub use qt_core::QFlags;
pub use qt_core::QObject;
pub use qt_core::Orientation as QOrientation;
pub use qt_core::QSize;
pub use qt_core::SlotNoArgs;
pub use qt_core::QString;
pub use qt_core::QVariant;
pub use qt_core::QBox;
pub use qt_core::QPtr;
pub use qt_core_custom_events::custom_event_filter::CustomEventFilter;
pub use qt_gui::{q_image::Format, QImage};
pub use qt_gui::QPixmap;
pub use qt_gui::QResizeEvent;
pub use qt_widgets::QMenu;
pub use qt_widgets::q_size_policy::Policy as QSizePolicy;
pub use qt_widgets::QWidget;
pub use qt_widgets::cpp_core::{CppBox, CppDeletable, DynamicCast, StaticUpcast, StaticDowncast, Ref, Ptr, NullPtr};
pub use std::ffi::CString;
pub use std::borrow::Cow;
pub use std::os::raw::c_void;
pub use std::{cmp, marker, mem, ops, ptr, sync::mpsc};

pub use plygui_api::sdk::*;
pub use plygui_api::{callbacks, controls, defaults, ids, layout, types::{self, adapter}, utils};
pub use plygui_api::external::image;

lazy_static! {
    pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
    pub static ref PROPERTY_PARENT: CString = CString::new("plygui_parent").unwrap();
}

pub enum MaybeCppBox<T: CppDeletable> {
    None,
    Some(CppBox<T>)
}
impl<T: CppDeletable> MaybeCppBox<T> {
    pub fn as_ref(&self) -> Ref<T> {
        match self {
            MaybeCppBox::None => panic!("CppBox is empty!"),
            MaybeCppBox::Some(ref cppb) => unsafe { cppb.as_ref() }
        }
    }
}
impl<T: CppDeletable> ops::Deref for MaybeCppBox<T> {
    type Target = CppBox<T>;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeCppBox::None => panic!("CppBox is empty!"),
            MaybeCppBox::Some(ref cppb) => cppb
        }
    }
}
impl<T: CppDeletable> ops::DerefMut for MaybeCppBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybeCppBox::None => panic!("CppBox is empty!"),
            MaybeCppBox::Some(ref mut cppb) => cppb
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QtId(ptr::NonNull<QObject>);

impl From<*mut QObject> for QtId {
    fn from(a: *mut QObject) -> QtId {
        QtId(ptr::NonNull::new(a).unwrap())
    }
}
impl From<QtId> for *mut QObject {
    fn from(a: QtId) -> *mut QObject {
        a.0.as_ptr()
    }
}
impl From<QtId> for usize {
    fn from(a: QtId) -> usize {
        a.0.as_ptr() as usize
    }
}
impl cmp::PartialOrd for QtId {
    fn partial_cmp(&self, other: &QtId) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl cmp::Ord for QtId {
    fn cmp(&self, other: &QtId) -> cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl ops::Deref for QtId {
    type Target = QObject;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl ops::DerefMut for QtId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl NativeId for QtId {
    unsafe fn from_outer(arg: usize) -> Self {
        QtId(ptr::NonNull::new(arg as *mut QObject).unwrap())
    }
}

#[repr(C)]
pub struct QtControlBase<T: controls::Control + Sized, Q: StaticUpcast<QWidget> + StaticUpcast<QObject> + CppDeletable> {
    pub widget: QBox<Q>,
    pub dirty: bool,

    event_callback: QBox<CustomEventFilter>,

    _marker: marker::PhantomData<T>,
}

impl<T: controls::Control + Sized, Q: StaticUpcast<QWidget> + StaticUpcast<QObject> + CppDeletable> QtControlBase<T, Q> {
    pub fn with_params<F>(widget: QBox<Q>, event_callback: F) -> QtControlBase<T, Q>
    where
        F: for<'a, 'b> FnMut(&'a mut QObject, &'b mut QEvent) -> bool,
    {
        let base = QtControlBase {
            widget: widget,
            event_callback: CustomEventFilter::new(event_callback),
            dirty: true,
            _marker: marker::PhantomData,
        };
        unsafe {
            base.widget.static_upcast::<QWidget>().set_minimum_size_2a(1, 1);
            let filter: *mut QObject = base.event_callback.static_upcast::<QObject>().as_mut_raw_ptr();
            let qobject: &QObject = &base.widget.static_upcast::<QWidget>();
            qobject.install_event_filter(filter);
        }
        base
    }
    pub fn as_qwidget(&self) -> QPtr<QWidget> {
        unsafe { self.widget.static_upcast::<QWidget>() }
    }
    pub fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        if let Some(_coords) = control.coords {
            //self.widget.static_upcast::<QWidget>().move_((coords.0 as i32, coords.1 as i32));
            let wpolicy = match control.layout.width {
                layout::Size::MatchParent => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_minimum_width(1); }
                    QSizePolicy::Expanding
                }
                layout::Size::WrapContent => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_minimum_width(1); }
                    QSizePolicy::Minimum
                }
                layout::Size::Exact(value) => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_fixed_width(value as i32); }
                    QSizePolicy::Fixed
                }
            };
            let hpolicy = match control.layout.height {
                layout::Size::MatchParent => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_minimum_height(1); }
                    QSizePolicy::Expanding
                }
                layout::Size::WrapContent => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_minimum_height(1); }
                    QSizePolicy::Minimum
                }
                layout::Size::Exact(value) => {
                    unsafe { self.widget.static_upcast::<QWidget>().set_fixed_height(value as i32); }
                    QSizePolicy::Fixed
                }
            };
            unsafe { self.widget.static_upcast::<QWidget>().set_size_policy_2a(wpolicy, hpolicy); }
        }
    }
    pub fn invalidate(&mut self) -> bool {
        let parent_widget = unsafe { self.widget.static_upcast::<QWidget>().parent_widget() };
        if unsafe { parent_widget.is_null() } {
            return false;
        }
        if let Some(mparent) = cast_qobject_to_base_mut(unsafe { &StaticUpcast::static_upcast(parent_widget.as_ptr()) }) {
            let (pw, ph) = mparent.as_member().is_has_size().unwrap().size();
            let this = cast_qobject_to_uimember_mut::<T>(unsafe { &self.widget.static_upcast::<QWidget>().static_upcast::<QWidget>() }).unwrap();
            let (_, _, changed) = this.measure(pw, ph);
            this.draw(None);
            
            if let Some(mparent) = mparent.as_member_mut().is_control_mut() {
                if changed && !mparent.is_skip_draw() {
                    mparent.invalidate();
                }
            }
        }
        true
    }
    pub fn set_visibility(&mut self, visibility: types::Visibility) {
        let widget = &mut self.widget;
        unsafe {
            let sp_retain = widget.static_upcast::<QWidget>().size_policy();
            sp_retain.set_retain_size_when_hidden(visibility != types::Visibility::Gone);
            widget.static_upcast::<QWidget>().set_size_policy_1a(&sp_retain);
            widget.static_upcast::<QWidget>().set_visible(visibility == types::Visibility::Visible);
        }
    }
    pub fn parent(&self) -> Option<&dyn controls::Member> {
        unsafe {
            let mut qv = self.widget.static_upcast::<QWidget>().parent_widget().as_ptr().static_upcast::<QObject>().property(PROPERTY.as_ptr() as *const i8);
            if qv.as_mut_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0 {
            	qv = self.widget.static_upcast::<QObject>().property(PROPERTY_PARENT.as_ptr() as *const i8);
            }
            if qv.as_mut_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0 {
            	None
            } else {
                Some(mem::transmute::<usize, &MemberBase>(qv.to_u_long_long_0a() as usize).as_member())
            }
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        unsafe {
            let mut qv = self.widget.static_upcast::<QWidget>().parent_widget().static_upcast::<QObject>().property(PROPERTY.as_ptr() as *const i8);
            if qv.as_mut_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0 {
            	qv = self.widget.static_upcast::<QObject>().property(PROPERTY_PARENT.as_ptr() as *const i8);
            }
            if qv.as_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0{
                None
            } else {
                Some(mem::transmute::<usize, &mut MemberBase>(qv.to_u_long_long_0a() as usize).as_member_mut())
            }
        }
    }
    pub fn root(&self) -> Option<&dyn controls::Member> {
        unsafe {
            let qv = self.widget.static_upcast::<QWidget>().window().as_ptr().static_upcast::<QObject>().property(PROPERTY.as_ptr() as *const i8);
            if qv.as_mut_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0{
                None
            } else {
                Some(mem::transmute::<usize, &MemberBase>(qv.to_u_long_long_0a() as usize).as_member())
            }
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        unsafe {
            let qv = self.widget.static_upcast::<QWidget>().window().static_upcast::<QObject>().property(PROPERTY.as_ptr() as *const i8);
            if qv.as_raw_ptr().is_null() || qv.to_u_long_long_0a() == 0 {
                None
            } else {
                Some(mem::transmute::<usize, &mut MemberBase>(qv.to_u_long_long_0a() as usize).as_member_mut())
            }
        }
    }
}

pub fn cast_control_to_qwidget_mut(control: &mut dyn controls::Control) -> &mut QWidget {
    cast_member_to_qwidget_mut(control.as_member_mut())
}
pub fn cast_control_to_qwidget(control: &dyn controls::Control) -> &QWidget {
    cast_member_to_qwidget(control.as_member())
}
pub fn cast_member_to_qwidget_mut(member: &mut dyn controls::Member) -> &mut QWidget {
    unsafe { &mut *(member.native_id() as *mut QWidget) }
}
pub fn cast_member_to_qwidget(member: &dyn controls::Member) -> &QWidget {
    unsafe { &*(member.native_id() as *const QWidget) }
}
pub unsafe fn cast_qobject_mut<'a, T>(object: &QObject) -> Option<&'a mut T>
where
    T: Sized,
{
    let qv = object.property(PROPERTY.as_ptr() as *const i8);
    if qv.as_mut_raw_ptr().is_null() {
        None
    } else {
        let ptr = qv.to_u_long_long_0a();
        Some(mem::transmute(ptr as usize))
    }
}
pub unsafe fn cast_qobject<'a, T>(object: &QObject) -> Option<&'a T>
where
    T: Sized,
{
    let qv = object.property(PROPERTY.as_ptr() as *const i8);
    if qv.as_raw_ptr().is_null() {
        None
    } else {
        let ptr = qv.to_u_long_long_0a();
        Some(mem::transmute(ptr as usize))
    }
}
pub fn cast_qobject_to_uimember_mut<'a, T>(object: &QObject) -> Option<&'a mut T>
where
    T: controls::Member + Sized,
{
    unsafe { cast_qobject_mut(object) }
}
pub fn cast_qobject_to_uimember<'a, T>(object: &QObject) -> Option<&'a T>
where
    T: controls::Member + Sized,
{
    unsafe { cast_qobject(object) }
}
pub fn cast_qobject_to_base_mut<'a>(object: &QObject) -> Option<&'a mut MemberBase> {
    unsafe { cast_qobject_mut(object) }
}
pub fn cast_qobject_to_base<'a>(object: &QObject) -> Option<&'a MemberBase> {
    unsafe { cast_qobject(object) }
}
pub fn orientation_to_qorientation(o: layout::Orientation) -> QOrientation {
    match o {
        layout::Orientation::Horizontal => QOrientation::Horizontal,
        layout::Orientation::Vertical => QOrientation::Vertical,
    }
}
pub fn qorientation_to_orientation(o: QOrientation) -> layout::Orientation {
    match o {
        QOrientation::Horizontal => layout::Orientation::Horizontal,
        QOrientation::Vertical => layout::Orientation::Vertical,
        _ => { unreachable!() }
    }
}
/*pub fn image_to_qimage(src: &image::DynamicImage) -> CppBox<QImage> {
    use image::GenericImageView;
    
    let (w, h) = src.dimensions();
    let raw = src.to_rgba().into_raw();
    unsafe { QImage::new_unsafe((raw.as_ptr(), w as i32, h as i32, Format::FormatRGBA8888)) }
}*/
pub fn append_item<T: controls::Member>(menu: &QMenu, label: String, action: callbacks::Action, storage: &mut Vec<(callbacks::Action, QBox<SlotNoArgs>)>, slot_spawn: fn(id: usize, selfptr: *mut T) -> QBox<SlotNoArgs>, selfptr: *mut T) {
    let id = storage.len();
    let action = (action, slot_spawn(id, selfptr));
    let qaction = unsafe { &*menu.add_action_q_string(QString::from_std_str(label).as_ref()) };
    unsafe { qaction.triggered().connect(&action.1) }; 
    storage.push(action);
}
pub fn append_level<T: controls::Member>(menu: &QMenu, label: String, items: Vec<types::MenuItem>, storage: &mut Vec<(callbacks::Action, QBox<SlotNoArgs>)>, slot_spawn: fn(id: usize, selfptr: *mut T) -> QBox<SlotNoArgs>, selfptr: *mut T) {
    let submenu = unsafe { menu.add_menu_q_string(QString::from_std_str(label).as_ref()) };
    make_menu(&submenu, items, storage, slot_spawn, selfptr);
}
pub fn make_menu<T: controls::Member>(menu: &QMenu, mut items: Vec<types::MenuItem>, storage: &mut Vec<(callbacks::Action, QBox<SlotNoArgs>)>, slot_spawn: fn(id: usize, selfptr: *mut T) -> QBox<SlotNoArgs>, selfptr: *mut T) {
    let mut options = Vec::new();
    let mut help = Vec::new();

    let make_special = |menu: &QMenu, mut special: Vec<types::MenuItem>, storage: &mut Vec<(callbacks::Action, QBox<SlotNoArgs>)>, slot_spawn: fn(id: usize, selfptr: *mut T) -> QBox<SlotNoArgs>, selfptr: *mut T| {
        for item in special.drain(..) {
            match item {
                types::MenuItem::Action(label, action, _) => {
                    append_item(menu, label, action, storage, slot_spawn, selfptr);
                }
                types::MenuItem::Sub(label, items, _) => {
                    append_level(menu, label, items, storage, slot_spawn, selfptr);
                }
                types::MenuItem::Delimiter => {
                    unsafe { menu.add_separator() };
                }
            }
        }
    };

    for item in items.drain(..) {
        match item {
            types::MenuItem::Action(label, action, role) => match role {
                types::MenuItemRole::None => {
                    append_item(menu, label, action, storage, slot_spawn, selfptr);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Action(label, action, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Action(label, action, role));
                }
            },
            types::MenuItem::Sub(label, items, role) => match role {
                types::MenuItemRole::None => {
                    append_level(menu, label, items, storage, slot_spawn, selfptr);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Sub(label, items, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Sub(label, items, role));
                }
            },
            types::MenuItem::Delimiter => {
                unsafe { menu.add_separator() };
            }
        }
    }

    make_special(menu, options, storage, slot_spawn, selfptr);
    make_special(menu, help, storage, slot_spawn, selfptr);
}
