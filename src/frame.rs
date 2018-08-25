use super::common::*;
use super::*;

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
    child: Option<Box<controls::Control>>,
}

impl FrameInner for QtFrame {
    fn with_label(label: &str) -> Box<Frame> {
        let mut fr = Box::new(Member::with_inner(
            Control::with_inner(
                SingleContainer::with_inner(
                    QtFrame {
                        base: common::QtControlBase::with_params(QGroupBox::new(&QString::from_std_str(label)), event_handler),
                        layout: QStackedLayout::new(),
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
        let qo = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
        let mut child = self.child.take();
        if let Some(ref mut child) = child {
            if let Some(self2) = common::cast_qobject_to_uimember_mut::<LinearLayout>(unsafe { &mut *qo }) {
                child.on_removed_from_container(self2);
            }
        }
        self.layout = CppBox::default();
    }
}
impl SingleContainerInner for QtFrame {
    fn set_child(&mut self, _base: &mut MemberBase, mut child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
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
        child
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

impl ContainerInner for QtFrame {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_by_id_mut(id);
            }
        }
        None
    }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control_by_id(id);
            }
        }
        None
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
        let margins = self.layout.as_ref().contents_margins();
        layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
    }
}

impl ControlInner for QtFrame {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        self.draw(member, control, Some((x, y)));
        let margins = self.base.widget.contents_margins();
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
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {
        if let Some(ref mut child) = self.child {
            let self2 = unsafe { utils::base_to_impl_mut::<Frame>(member) };
            child.on_removed_from_container(self2);
        }
    }

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
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

impl MemberInner for QtFrame {
    type Id = common::QtId;

    fn on_set_visibility(&mut self, _base: &mut MemberBase) {
        self.base.invalidate()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.as_ref().static_cast() as *const QWidget as *mut QWidget)
    }
}

impl Drawable for QtFrame {
    fn draw(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(coords);
        if let Some(ref mut child) = self.child {
            child.draw(Some((0, 0)));
        }
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        let (lm, tm, rm, bm) = self.layout_margin(member).into();
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = self.base.widget.as_ref().font();

                let mut label_size = QRect::new((0, 0, 0, 0));
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.width() < 1 {
                            let mut fm = QFontMetrics::new(font);
                            label_size = fm.bounding_rect(&self.base.widget.as_ref().title());
                        }
                        if let Some(ref mut child) = self.child {
                            let (cw, _, _) = child.measure(cmp::max(0, parent_width as i32 - lm + 2) as u16, cmp::max(0, parent_height as i32 - tm - bm) as u16);
                            let w = label_size.width();
                            label_size.set_width(w + cw as i32 + lm + rm);
                            measured = true;
                        }
                        label_size.width()
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.height() < 1 {
                            let mut fm = QFontMetrics::new(font);
                            label_size = fm.bounding_rect(&self.base.widget.as_ref().title());
                        }
                        if let Some(ref mut child) = self.child {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(cmp::max(0, parent_width as i32 - lm + 2) as u16, cmp::max(0, parent_height as i32 - tm - bm) as u16);
                                ch
                            };
                            let h = label_size.height();
                            label_size.set_height(h + ch as i32 + tm + bm);
                        }
                        label_size.height()
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Frame::with_label("").into_control()
}

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(fr) = cast_qobject_to_uimember_mut::<Frame>(object) {
                use plygui_api::controls::Member;

                if fr.as_inner().as_inner().as_inner().base.dirty {
                    fr.as_inner_mut().as_inner_mut().as_inner_mut().base.dirty = false;
                    let (width, height) = fr.size();
                    fr.call_on_resize(width, height);
                }
            }
        }
        _ => {}
    }
    false
}

impl_all_defaults!(Frame);
