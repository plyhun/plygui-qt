pub use qt_core::connection::Signal;
pub use qt_core::cpp_utils::{CppBox, CppDeletable, DynamicCast, StaticCast};
pub use qt_core::event::{Event as QEvent, Type as QEventType};
pub use qt_core::flags::Flags;
pub use qt_core::object::Object as QObject;
pub use qt_core::qt::Orientation as QOrientation;
pub use qt_core::size::Size as QSize;
pub use qt_core::string::String as QString;
pub use qt_core::variant::Variant as QVariant;
pub use qt_core_custom_events::custom_event_filter::CustomEventFilter;
pub use qt_widgets::size_policy::Policy as QPolicy;
pub use qt_widgets::widget::Widget as QWidget;

pub use std::ffi::CString;
pub use std::os::raw::c_void;
pub use std::{cmp, marker, mem, ops, ptr};

pub use plygui_api::development::*;
pub use plygui_api::{callbacks, controls, defaults, ids, layout, types, utils};

lazy_static! {
    pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QtId(ptr::NonNull<QWidget>);

impl From<*mut QWidget> for QtId {
    fn from(a: *mut QWidget) -> QtId {
        QtId(ptr::NonNull::new(a).unwrap())
    }
}
impl From<QtId> for *mut QWidget {
    fn from(a: QtId) -> *mut QWidget {
        a.0.as_ptr()
    }
}
impl From<QtId> for usize {
    fn from(a: QtId) -> usize {
        a.0.as_ptr() as usize
    }
}
impl From<usize> for QtId {
    fn from(a: usize) -> QtId {
        QtId(ptr::NonNull::new(a as *mut QWidget).unwrap())
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
    type Target = QWidget;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}
impl ops::DerefMut for QtId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}
impl NativeId for QtId {}

#[repr(C)]
pub struct QtControlBase<T: controls::Control + Sized, Q: StaticCast<QWidget> + CppDeletable> {
    pub widget: CppBox<Q>,
    pub coords: Option<(i32, i32)>,
    pub measured_size: (u16, u16),
    pub dirty: bool,

    event_callback: CppBox<CustomEventFilter>,

    _marker: marker::PhantomData<T>,
}

impl<T: controls::Control + Sized, Q: StaticCast<QWidget> + CppDeletable> QtControlBase<T, Q> {
    pub fn with_params<F>(widget: CppBox<Q>, event_callback: F) -> QtControlBase<T, Q>
    where
        F: for<'a, 'b> FnMut(&'a mut QObject, &'b QEvent) -> bool,
    {
        let mut base = QtControlBase {
            widget: widget,
            event_callback: CustomEventFilter::new(event_callback),
            coords: None,
            measured_size: (0, 0),
            dirty: true,
            _marker: marker::PhantomData,
        };
        //base.widget.as_mut().static_cast_mut().set_size_policy((QPolicy::Fixed, QPolicy::Fixed));
        base.widget.as_mut().static_cast_mut().set_minimum_size((1, 1));
        unsafe {
            let filter: *mut QObject = base.event_callback.static_cast_mut() as *mut QObject;
            let qobject: &mut QObject = base.widget.as_mut().static_cast_mut().static_cast_mut();
            qobject.install_event_filter(filter);
        }
        base
    }
    pub fn as_qwidget(&self) -> &QWidget {
        self.widget.as_ref().static_cast()
    }
    pub fn as_qwidget_mut(&mut self) -> &mut QWidget {
        self.widget.as_mut().static_cast_mut()
    }
    pub fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        if coords.is_some() {
            self.coords = coords;
        }
        if let Some(coords) = self.coords {
            self.widget.static_cast_mut().move_((coords.0 as i32, coords.1 as i32));
            match control.layout.width {
                layout::Size::MatchParent => {
                    self.widget.static_cast_mut().set_minimum_width(1);
                }
                _ => {
                    self.widget.static_cast_mut().set_fixed_width(self.measured_size.0 as i32);
                }
            }
            match control.layout.height {
                layout::Size::MatchParent => {
                    self.widget.static_cast_mut().set_minimum_height(1);
                }
                _ => {
                    self.widget.static_cast_mut().set_fixed_height(self.measured_size.1 as i32);
                }
            }
        }
    }
    pub fn invalidate(&mut self) {
        use qt_core::cpp_utils::StaticCast;

        let parent_widget = self.widget.as_mut().static_cast_mut().parent_widget();
        if parent_widget.is_null() {
            return;
        }
        if let Some(mparent) = cast_qobject_to_base_mut((unsafe { &mut *parent_widget }).static_cast_mut() as &mut QObject) {
            let (pw, ph) = mparent.as_member().size();
            let this = cast_qobject_to_uimember_mut::<T>(self.widget.as_mut().static_cast_mut().static_cast_mut()).unwrap();
            let (_, _, changed) = this.measure(pw, ph);
            this.draw(None);

            if let Some(mparent) = mparent.as_member_mut().is_control_mut() {
                if changed && !mparent.is_skip_draw() {
                    mparent.invalidate();
                }
            }
        }
    }
    pub fn set_visibility(&mut self, visibility: types::Visibility) {
        let widget = self.widget.as_mut();
        let mut sp_retain = widget.static_cast_mut().size_policy();
        sp_retain.set_retain_size_when_hidden(visibility != types::Visibility::Gone);
        widget.static_cast_mut().set_size_policy(&sp_retain);
        widget.static_cast_mut().set_visible(visibility == types::Visibility::Visible);
    }
    pub fn parent(&self) -> Option<&controls::Member> {
        unsafe {
            let ptr = ((&*self.widget.as_ref().static_cast().parent_widget()).static_cast() as &QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
                let m: &MemberBase = mem::transmute(ptr);
                Some(m.as_member())
            } else {
                None
            }
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        unsafe {
            let ptr = ((&mut *self.widget.as_mut().static_cast_mut().parent_widget()).static_cast_mut() as &mut QObject)
                .property(PROPERTY.as_ptr() as *const i8)
                .to_u_long_long();
            if ptr != 0 {
                let m: &mut MemberBase = mem::transmute(ptr);
                Some(m.as_member_mut())
            } else {
                None
            }
        }
    }
    pub fn root(&self) -> Option<&controls::Member> {
        unsafe {
            let ptr = ((&*self.widget.as_ref().static_cast().window()).static_cast() as &QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
                let m: &MemberBase = mem::transmute(ptr);
                Some(m.as_member())
            } else {
                None
            }
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut controls::Member> {
        unsafe {
            let ptr = ((&mut *self.widget.as_mut().static_cast_mut().window()).static_cast_mut() as &mut QObject).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
            if ptr != 0 {
                let m: &mut MemberBase = mem::transmute(ptr);
                Some(m.as_member_mut())
            } else {
                None
            }
        }
    }
}

pub fn cast_control_to_qwidget_mut(control: &mut controls::Control) -> &mut QWidget {
    unsafe { &mut *(control.native_id() as *mut QWidget) }
}
pub fn cast_control_to_qwidget(control: &controls::Control) -> &QWidget {
    unsafe { &*(control.native_id() as *const QWidget) }
}
fn cast_qobject_mut<'a, T>(object: &'a mut QObject) -> Option<&'a mut T>
where
    T: Sized,
{
    unsafe {
        let ptr = (&*object).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
        if ptr != 0 {
            Some(::std::mem::transmute(ptr))
        } else {
            None
        }
    }
}
fn cast_qobject<'a, T>(object: &'a QObject) -> Option<&'a T>
where
    T: Sized,
{
    unsafe {
        let ptr = (&*object).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
        if ptr != 0 {
            Some(::std::mem::transmute(ptr))
        } else {
            None
        }
    }
}
pub fn cast_qobject_to_uimember_mut<'a, T>(object: &'a mut QObject) -> Option<&'a mut T>
where
    T: controls::Member + Sized,
{
    cast_qobject_mut(object)
}
pub fn cast_qobject_to_uimember<'a, T>(object: &'a QObject) -> Option<&'a T>
where
    T: controls::Member + Sized,
{
    cast_qobject(object)
}
pub fn cast_qobject_to_base_mut<'a>(object: &'a mut QObject) -> Option<&'a mut MemberBase> {
    cast_qobject_mut(object)
}
pub fn cast_qobject_to_base<'a>(object: &'a QObject) -> Option<&'a MemberBase> {
    cast_qobject(object)
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
    }
}
