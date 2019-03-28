use crate::common::{self, *};

use qt_core::rect::Rect as QRect;
use qt_gui::font_metrics::FontMetrics as QFontMetrics;
use qt_widgets::group_box::GroupBox as QGroupBox;
use qt_widgets::stacked_layout::StackedLayout as QStackedLayout;

use std::borrow::Cow;

pub type Frame = Member<Control<SingleContainer<QtFrame>>>;

#[repr(C)]
pub struct QtFrame {
    base: common::QtControlBase<Frame, QGroupBox>,
    layout: CppBox<QStackedLayout>,
    label_size: QRect,
    child: Option<Box<dyn controls::Control>>,
}

impl FrameInner for QtFrame {
    fn with_label(label: &str) -> Box<Frame> {
        let mut fr = Box::new(Member::with_inner(
            Control::with_inner(
                SingleContainer::with_inner(
                    QtFrame {
                        base: common::QtControlBase::with_params(QGroupBox::new(&QString::from_std_str(label)), event_handler),
                        layout: QStackedLayout::new(),
                        label_size: QRect::new((0, 0, 0, 0)),
                        child: None,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let fr1 = fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.as_mut() as *mut QGroupBox;
            (&mut *fr1).set_layout(fr.as_inner_mut().as_inner_mut().as_inner_mut().layout.static_cast_mut());

            let ptr = fr.as_ref() as *const _ as u64;
            let qo: &mut QObject = fr.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        fr
    }
}
impl Drop for QtFrame {
    fn drop(&mut self) {
        if !self.base.widget.is_null() {
            let qo = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
            let mut child = self.child.take();
            if let Some(ref mut child) = child {
                if let Some(self2) = common::cast_qobject_to_uimember_mut::<Frame>(unsafe { &mut *qo }) {
                    child.on_removed_from_container(self2);
                }
            }
            //self.layout = CppBox::default();
        }
    }
}
impl SingleContainerInner for QtFrame {
    fn set_child(&mut self, _base: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        mem::swap(&mut child, &mut self.child);
        if let Some(old) = child.as_mut() {
            unsafe {
                (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QStackedLayout).remove_widget(common::cast_control_to_qwidget_mut(old.as_mut()));
            }
        }
        if let Some(new) = self.child.as_mut() {
            unsafe {
                (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QStackedLayout).add_widget(common::cast_control_to_qwidget_mut(new.as_mut()) as *mut QWidget);
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
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if child.as_member().id() == id {
                Some(child.as_mut())
            } else if let Some(c) = child.is_container_mut() {
                c.find_control_by_id_mut(id)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if child.as_member().id() == id {
                Some(child.as_ref())
            } else if let Some(c) = child.is_container() {
                c.find_control_by_id(id)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl HasLabelInner for QtFrame {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let name = self.base.widget.as_ref().title().to_utf8();
        unsafe {
            let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.base.widget.as_mut().set_title(&QString::from_std_str(label));
    }
}

impl HasLayoutInner for QtFrame {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        let margins = self.layout.contents_margins();
        layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
    }
}

impl ControlInner for QtFrame {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.draw(member, control);
        let margins = self.layout.contents_margins();
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            child.on_added_to_container(
                self2,
                0,
                0,
                utils::coord_to_size(cmp::max(0, pw as i32 - margins.left() - margins.right())),
                utils::coord_to_size(cmp::max(0, ph as i32 - margins.top() - margins.bottom())),
            );
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

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
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
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for QtFrame {}

impl Drawable for QtFrame {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
        let margins = self.base.widget.contents_margins();
        if let Some(ref mut child) = self.child {
            child.draw(Some((margins.left(), margins.top())));
        }
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        let (lm, tm, rm, bm) = self.layout_margin(member).into();
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = self.base.widget.as_ref().font();

                let mut measured = false;
                self.label_size = QRect::new((0, 0, 0, 0));
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        if self.label_size.width() < 1 {
                            let fm = QFontMetrics::new(font);
                            self.label_size = fm.bounding_rect(&self.base.widget.as_ref().title());
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
                        if self.label_size.height() < 1 {
                            let fm = QFontMetrics::new(font);
                            self.label_size = fm.bounding_rect(&self.base.widget.as_ref().title());
                        }
                        if let Some(ref mut child) = self.child {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(cmp::max(0, parent_width as i32 - lm - rm) as u16, cmp::max(0, parent_height as i32 - tm - bm) as u16);
                                ch
                            };
                            h = self.label_size.height() + ch as i32 + tm + bm;
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

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Frame::with_label("").into_control()
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(fr) = cast_qobject_to_uimember_mut::<Frame>(object) {
                use plygui_api::controls::HasSize;

                if fr.as_inner().as_inner().as_inner().base.dirty {
                    fr.as_inner_mut().as_inner_mut().as_inner_mut().base.dirty = false;
                    let (width, height) = fr.size();
                    fr.call_on_size(width, height);
                }
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Frame>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut()));
                }
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().as_inner_mut().layout, CppBox::new(ptr::null_mut()));
                }
            }
        }
        _ => {}
    }
    false
}

default_impls_as!(Frame);
