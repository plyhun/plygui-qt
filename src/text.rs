use crate::common::{self, *};

use qt_core::QRect;
use qt_gui::QFontMetrics;
use qt_widgets::QLabel;

use std::borrow::Cow;
use std::cmp::max;

pub type Text = AMember<AControl<AText<QtText>>>;

#[repr(C)]
pub struct QtText {
    base: common::QtControlBase<Text, QLabel>,
}

impl HasLabelInner for QtText {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = self.base.widget.as_ref().text().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data().as_raw_ptr() as *const u8, name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.base.widget.set_text(&QString::from_std_str(label)) };
    }
}

impl<O: controls::Text> NewTextInner<O> for QtText {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let mut t = QtText {
            base: common::QtControlBase::with_params(unsafe { QLabel::new() }, event_handler::<O>),
        };
        unsafe {
            let ptr = ptr as *const _ as u64;
            let mut qo = t.base.widget.static_upcast_mut::<QObject>();
            qo.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        t
    }
}
impl TextInner for QtText {
    fn with_text<S: AsRef<str>>(text: S) -> Box<dyn controls::Text> {
        let mut b: Box<mem::MaybeUninit<Text>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AText::with_inner(
                    <Self as NewTextInner<Text>>::with_uninit(b.as_mut()),
                )
            ),
        );
        controls::HasLabel::set_label(&mut ab, text.as_ref().into());
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl HasLayoutInner for QtText {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtText {
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut()
    }
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {}

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_TEXT;

        fill_from_markup_base!(self, markup, registry, Text, [MEMBER_ID_TEXT, MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, markup);
        fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut controls::Text)]);
    }
}

impl HasNativeIdInner for QtText {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl HasVisibilityInner for QtText {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtText {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtText {}

impl Drawable for QtText {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = unsafe { self.base.widget.as_ref().font() };
                let mut label_size = unsafe { QRect::from_4_int(0, 0, 0, 0) };
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => unsafe {
                        if label_size.width() < 1 {
                            let fm = QFontMetrics::new_1a(font);
                            label_size = fm.bounding_rect_q_string(&self.base.widget.as_ref().text());
                        }
                        label_size.width() + 16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => unsafe {
                        if label_size.height() < 1 {
                            let fm = QFontMetrics::new_1a(font);
                            label_size = fm.bounding_rect_q_string(&self.base.widget.as_ref().text());
                        }
                        label_size.height() + 16
                    }
                };
                (max(0, w) as u16, max(0, h) as u16)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}
impl Spawnable for QtText {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_text("").into_control()
    }
}

fn event_handler<O: controls::Text>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Text>(object) {
                let size = unsafe { 
                    let size = &mut MutRef::from_raw_ref(event).static_downcast_mut::<QResizeEvent>();
                    let size = (
                    	utils::coord_to_size(size.size().width()), 
                    	utils::coord_to_size(size.size().height())
                    );
                    this.inner_mut().base.measured = size;
                    if let layout::Size::WrapContent = this.inner_mut().base.layout.width {
                        this.inner_mut().inner_mut().inner_mut().base.widget.set_minimum_width(size.0 as i32);  
                        this.inner_mut().inner_mut().inner_mut().base.widget.set_maximum_width(size.0 as i32); 
                    }
                    if let layout::Size::WrapContent = this.inner_mut().base.layout.height {
                        this.inner_mut().inner_mut().inner_mut().base.widget.set_minimum_height(size.1 as i32); 
                        this.inner_mut().inner_mut().inner_mut().base.widget.set_maximum_height(size.1 as i32); 
                    }
                    size
                };
                this.call_on_size::<O>(size.0, size.1);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Text>(object) {
                unsafe {
                    ptr::write(&mut ll.inner_mut().inner_mut().inner_mut().base.widget, common::MaybeCppBox::None);
                }
            }
        }
        _ => {}
    }
    false
}
