use crate::common::{self, *};

use qt_core::QRect;
use qt_gui::QFontMetrics;
use qt_widgets::QGroupBox;
use qt_widgets::QStackedLayout;
use qt_widgets::QLayout;

use std::borrow::Cow;

pub type Frame = AMember<AControl<AContainer<ASingleContainer<AFrame<QtFrame>>>>>;

#[repr(C)]
pub struct QtFrame {
    base: common::QtControlBase<Frame, QGroupBox>,
    layout: QBox<QStackedLayout>,
    label_size: CppBox<QRect>,
    child: Option<Box<dyn controls::Control>>,
}
impl<O: controls::Frame> NewFrameInner<O> for QtFrame {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let fr = QtFrame {
            base: common::QtControlBase::with_params(unsafe { QGroupBox::from_q_string(&QString::new()) }, event_handler::<O>),
            layout: unsafe { QStackedLayout::new() },
            label_size: unsafe { QRect::from_4_int(0, 0, 0, 0) },
            child: None,
        };
        unsafe {
            let fr1 = fr.base.widget.as_mut_raw_ptr();
            (&mut *fr1).set_layout(fr.layout.as_ptr().static_upcast::<QLayout>());

            let ptr = ptr as *mut _ as u64;
            let qo: &QObject = &fr.base.widget.as_ptr().static_upcast();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        fr
    }
}
impl FrameInner for QtFrame {
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn controls::Frame> {
        let mut b: Box<mem::MaybeUninit<Frame>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    ASingleContainer::with_inner(
                        AFrame::with_inner(
                            <Self as NewFrameInner<Frame>>::with_uninit(b.as_mut())
                        )
                    ),
                ),
            ),
        );
        controls::HasLabel::set_label(&mut ab, label.as_ref().into());
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}
impl SingleContainerInner for QtFrame {
    fn set_child(&mut self, _base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        mem::swap(&mut child, &mut self.child);
        if let Some(old) = child.as_mut() {
            unsafe {
                (self.base.widget.layout().static_downcast::<QStackedLayout>()).remove_widget(Ptr::from_raw(common::cast_control_to_qwidget_mut(old.as_mut())));
            }
        }
        if let Some(new) = self.child.as_mut() {
            unsafe {
                (self.base.widget.layout().static_downcast::<QStackedLayout>()).add_widget(Ptr::from_raw(common::cast_control_to_qwidget_mut(new.as_mut())));
            }
        }
        self.base.invalidate();
        child
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

impl ContainerInner for QtFrame {
    fn find_control_mut<'a>(&'a mut self, arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            match arg {
                types::FindBy::Id(id) => {
                    if child.as_member_mut().id() == id {
                        return Some(child.as_mut());
                    }
                }
                types::FindBy::Tag(tag) => {
                    if let Some(mytag) = child.as_member_mut().tag() {
                        if tag == mytag {
                            return Some(child.as_mut());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container_mut() {
                c.find_control_mut(arg)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn find_control<'a>(&'a self, arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            match arg {
                types::FindBy::Id(id) => {
                    if child.as_member().id() == id {
                        return Some(child.as_ref());
                    }
                }
                types::FindBy::Tag(tag) => {
                    if let Some(mytag) = child.as_member().tag() {
                        if tag == mytag {
                            return Some(child.as_ref());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container() {
                c.find_control(arg)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl HasLabelInner for QtFrame {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = self.base.widget.title().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data(), name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(mem::transmute(bytes)).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.base.widget.set_title(&QString::from_std_str(&label)) };
    }
}

impl HasLayoutInner for QtFrame {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        unsafe { 
            let margins = self.layout.contents_margins();
            layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
        }
    }
}

impl ControlInner for QtFrame {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.draw(member, control);
        let margins = unsafe { self.layout.contents_margins() };
        if let Some(ref mut child) = self.child {
            unsafe { 
                let self2 = utils::base_to_impl_mut::<Frame>(member);
                child.on_added_to_container(
                    self2,
                    0,
                    0,
                    utils::coord_to_size(cmp::max(0, pw as i32 - margins.left() - margins.right())),
                    utils::coord_to_size(cmp::max(0, ph as i32 - margins.top() - margins.bottom())),
                );
            }
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            child.on_removed_from_container(self2);
        }
    }

    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fifr_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fifr_from_markup_base!(self, base, markup, registry, Frame, [MEMBER_TYPE_BUTTON]);
        fifr_from_markup_label!(self, &mut base.member, markup);
        fifr_from_markup_cafrbacks!(self, markup, registry, [on_click => plygui_api::cafrbacks::Click]);
    }
}

impl HasNativeIdInner for QtFrame {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.base.widget.static_upcast::<QObject>().as_raw_ptr() } as *mut QObject)
    }
}
impl HasVisibilityInner for QtFrame {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtFrame {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtFrame {}

impl Drawable for QtFrame {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
        /*let margins = self.base.widget.contents_margins();
        if let Some(ref mut child) = self.child {
            child.draw(Some((margins.left(), margins.top() + self.label_size.height())));
        }*/
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        let (lm, tm, rm, bm) = self.layout_margin(member).into();
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = unsafe { self.base.widget.font() };

                let mut measured = false;
                self.label_size = unsafe { QRect::from_4_int(0, 0, 0, 0) };
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        unsafe {
                            if self.label_size.width() < 1 {
                                let fm = QFontMetrics::new_1a(font);
                                self.label_size = fm.bounding_rect_q_string(&self.base.widget.title());
                            }
                        }
                        if let Some(ref mut child) = self.child {
                            let (cw, _, _) = child.measure(cmp::max(0, parent_width as i32 - lm - rm) as u16, cmp::max(0, parent_height as i32 - tm - bm) as u16);
                            w = cw as i32 + lm + rm;
                            measured = true;
                        }
                        w
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        let mut h = 0;
                        unsafe {
                            if self.label_size.height() < 1 {
                                let fm = QFontMetrics::new_1a(font);
                                self.label_size = fm.bounding_rect_q_string(&self.base.widget.title());
                            }
                        }
                        if let Some(ref mut child) = self.child {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(cmp::max(0, parent_width as i32 - lm - rm) as u16, cmp::max(0, parent_height as i32 - tm - bm) as u16);
                                ch
                            };
                            h = unsafe { self.label_size.height() + ch as i32 + tm + bm };
                        }
                        h
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl Spawnable for QtFrame {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_label("").into_control()
    }
}

fn event_handler<O: controls::Frame>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Frame>(object) {
                unsafe { 
                    let size = Ref::from_raw(event).unwrap().static_downcast::<QResizeEvent>();
                    let size = (
                    	utils::coord_to_size(size.size().width()), 
                    	utils::coord_to_size(size.size().height())
                    );
                    this.inner_mut().base.measured = size;
                    this.call_on_size::<O>(size.0, size.1);
                }
            }
        }
        _ => {}
    }
    false
}
