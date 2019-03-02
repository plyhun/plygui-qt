use super::common::*;
use super::*;

use qt_widgets::box_layout::{BoxLayout as QBoxLayout, Direction};
use qt_widgets::frame::Frame as QFrame;

pub type LinearLayout = Member<Control<MultiContainer<QtLinearLayout>>>;

#[repr(C)]
pub struct QtLinearLayout {
    base: common::QtControlBase<LinearLayout, QFrame>,
    layout: CppBox<QBoxLayout>,
    children: Vec<Box<dyn controls::Control>>,
}

impl LinearLayoutInner for QtLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<LinearLayout> {
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                MultiContainer::with_inner(
                    QtLinearLayout {
                        base: common::QtControlBase::with_params(QFrame::new(), event_handler),
                        layout: QBoxLayout::new(orientation_to_box_direction(orientation)),
                        children: Vec::new(),
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ll1 = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.as_mut() as *mut QFrame;
            (&mut *ll1).set_layout(ll.as_inner_mut().as_inner_mut().as_inner_mut().layout.static_cast_mut());

            let ptr = ll.as_ref() as *const _ as u64;
            let qo: &mut QObject = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        ll
    }
}

impl Drop for QtLinearLayout {
    fn drop(&mut self) {
        if !self.base.widget.is_null() {
            let qo = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
            if let Some(self2) = common::cast_qobject_to_uimember_mut::<LinearLayout>(unsafe { &mut *qo }) {
                for mut child in self.children.drain(..) {
                    child.on_removed_from_container(self2);
                }
            }
            //self.layout = CppBox::default();
        }
    }
}

impl HasNativeIdInner for QtLinearLayout {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
    }
}
impl HasVisibilityInner for QtLinearLayout {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtLinearLayout {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for QtLinearLayout {}

impl Drawable for QtLinearLayout {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);

        let orientation = self.layout_orientation();
        let margins = self.layout.contents_margins();
        let spacing = self.layout.as_ref().spacing();
        let mut x = margins.left();
        let mut y = margins.top();
        for child in self.children.as_mut_slice() {
            child.draw(Some((x, y)));
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32 + spacing,
                layout::Orientation::Vertical => y += yy as i32 + spacing,
            }
        }
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

        let orientation = self.layout_orientation();
        let old_size = control.measured;
        let (lm, tm, rm, bm) = self.layout_margin(member).into();
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        let spacing = self.layout.as_ref().spacing();
                        let mut w = 0;
                        for child in self.children.as_mut_slice() {
                            let (cw, _, _) = child.measure(max(0, parent_width as i32 - lm - rm - spacing - spacing) as u16, max(0, parent_height as i32 - tm - bm) as u16);
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    w = max(w, cw);
                                }
                            }
                            w += match child.visibility() {
                                types::Visibility::Gone => 0,
                                _ => max(0, spacing) as u16,
                            };
                        }
                        measured = true;
                        max(0, w as i32 + lm + rm + spacing) as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        let spacing = self.layout.as_ref().spacing();
                        let mut h = 0;
                        for child in self.children.as_mut_slice() {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(max(0, parent_width as i32 - lm - rm - spacing - spacing) as u16, max(0, parent_height as i32 - tm - bm) as u16);
                                ch
                            };
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    h = max(h, ch);
                                }
                                layout::Orientation::Vertical => {
                                    h += ch;
                                }
                            }
                            h += match child.visibility() {
                                types::Visibility::Gone => 0,
                                _ => max(0, spacing) as u16,
                            };
                        }
                        max(0, h as i32 + tm + bm + spacing) as u16
                    }
                };
                (w, h)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl HasLayoutInner for QtLinearLayout {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        let margins = self.layout.contents_margins();
        layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
    }
}

impl ControlInner for QtLinearLayout {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
        let margins = self.layout.contents_margins();

        let orientation = self.layout_orientation();
        let mut x = margins.left();
        let mut y = margins.top();
        let pw = utils::coord_to_size(cmp::max(0, pw as i32 - margins.left() - margins.right()));
        let ph = utils::coord_to_size(cmp::max(0, ph as i32 - margins.top() - margins.bottom()));
        let spacing = self.layout.as_ref().spacing();
        for child in self.children.as_mut_slice() {
            let self2: &mut LinearLayout = unsafe { utils::base_to_impl_mut(member) };
            child.on_added_to_container(self2, x, y, pw, ph);
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32 + spacing,
                layout::Orientation::Vertical => y += yy as i32 + spacing,
            }
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let self2: &mut LinearLayout = unsafe { utils::base_to_impl_mut(member) };
        for child in self.children.iter_mut() {
            child.on_removed_from_container(self2);
        }
    }

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
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;

        fill_from_markup_base!(self, markup, registry, LinearLayout, [MEMBER_ID_LAYOUT_LINEAR, MEMBER_TYPE_LINEAR_LAYOUT]);
        fill_from_markup_children!(self, markup, registry);
    }
}

impl HasOrientationInner for QtLinearLayout {
    fn layout_orientation(&self) -> layout::Orientation {
        box_direction_to_orientation((unsafe { self.base.widget.as_ref().layout().as_ref() }.unwrap().dynamic_cast().unwrap() as &QBoxLayout).direction())
    }
    fn set_layout_orientation(&mut self, _base: &mut MemberBase, orientation: layout::Orientation) {
        unsafe { (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QBoxLayout) }.set_direction(orientation_to_box_direction(orientation));
        self.base.invalidate();
    }
}

impl ContainerInner for QtLinearLayout {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn controls::Control> {
        for child in self.children.as_mut_slice() {
            if child.as_member().id() == id {
                return Some(child.as_mut());
            } else if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_by_id_mut(id);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&dyn controls::Control> {
        for child in self.children.as_slice() {
            if child.as_member().id() == id {
                return Some(child.as_ref());
            } else if let Some(c) = child.is_container() {
                let ret = c.find_control_by_id(id);
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
}

impl MultiContainerInner for QtLinearLayout {
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, _base: &mut MemberBase, index: usize, mut child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let old = if index < self.children.len() {
            mem::swap(&mut child, &mut self.children[index]);
            unsafe {
                (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QBoxLayout).remove_widget(common::cast_control_to_qwidget_mut(child.as_mut()));
            }
            Some(child)
        } else {
            self.children.insert(index, child);
            None
        };
        unsafe {
            (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QBoxLayout).insert_widget((index as i32, common::cast_control_to_qwidget_mut(self.children[index].as_mut()) as *mut QWidget));
        }
        self.base.invalidate();
        old
    }
    fn remove_child_from(&mut self, _base: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>> {
        if index < self.children.len() {
            let mut item = self.children.remove(index);
            unsafe {
                (self.base.widget.as_mut().layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QBoxLayout).remove_widget(common::cast_control_to_qwidget_mut(item.as_mut()));
            }
            self.base.invalidate();
            Some(item)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        self.children.get(index).map(|c| c.as_ref())
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        //self.children.get_mut(index).map(|c| c.as_mut()) //the anonymous lifetime #1 does not necessarily outlive the static lifetime
        if let Some(c) = self.children.get_mut(index) {
            Some(c.as_mut())
        } else {
            None
        }
    }
}

fn box_direction_to_orientation(a: Direction) -> layout::Orientation {
    match a {
        Direction::TopToBottom => layout::Orientation::Vertical,
        Direction::LeftToRight => layout::Orientation::Horizontal,
        _ => unreachable!(),
    }
}
fn orientation_to_box_direction(a: layout::Orientation) -> Direction {
    match a {
        layout::Orientation::Horizontal => Direction::LeftToRight,
        layout::Orientation::Vertical => Direction::TopToBottom,
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    LinearLayout::with_orientation(layout::Orientation::Vertical).into_control()
}

impl_all_defaults!(LinearLayout);

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<LinearLayout>(object) {
                use plygui_api::controls::HasSize;

                let (width, height) = ll.size();
                ll.call_on_size(width, height);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<LinearLayout>(object) {
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
