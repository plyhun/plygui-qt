use crate::common::{self, *};

use qt_widgets::{QBoxLayout, q_box_layout::Direction as QDirection};
use qt_widgets::QLayout;
use qt_widgets::QSpacerItem;
use qt_widgets::QFrame;

pub type LinearLayout = AMember<AControl<AContainer<AMultiContainer<ALinearLayout<QtLinearLayout>>>>>;

#[repr(C)]
pub struct QtLinearLayout {
    base: common::QtControlBase<LinearLayout, QFrame>,
    layout: common::MaybeCppBox<QBoxLayout>,
    children: Vec<Box<dyn controls::Control>>,
}

impl<O: controls::LinearLayout> NewLinearLayoutInner<O> for QtLinearLayout {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let mut ll = QtLinearLayout {
            base: common::QtControlBase::with_params(unsafe { QFrame::new_0a() }, event_handler::<O>),
            layout: common::MaybeCppBox::Some(unsafe { QBoxLayout::new_1a(QDirection::TopToBottom) }),
            children: Vec::new(),
        };
        unsafe {
            let ll1 = ll.base.widget.as_mut_raw_ptr() as *mut QFrame;
            let layout = ll.layout.static_upcast_mut::<QLayout>();
            //let layout = layout as *mut QLayout;
            (&mut *ll1).set_layout(layout.as_mut_ptr());

            let ptr = ptr as *const _ as u64;
            let mut qo = ll.base.widget.static_upcast_mut::<QObject>();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        ll
    }
}
impl LinearLayoutInner for QtLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<dyn controls::LinearLayout> {
        let mut b: Box<mem::MaybeUninit<LinearLayout>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AMultiContainer::with_inner(
                        ALinearLayout::with_inner(
                            <Self as NewLinearLayoutInner<LinearLayout>>::with_uninit(b.as_mut()),
                        )
                    ),
                )
            ),
        );
        controls::HasOrientation::set_orientation(&mut ab, orientation);
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}

impl Drop for QtLinearLayout {
    fn drop(&mut self) {
        self.children.clear();
    }
}

impl HasNativeIdInner for QtLinearLayout {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
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
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtLinearLayout {}

impl Drawable for QtLinearLayout {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

        let orientation = self.orientation(member);
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
                        let spacing = unsafe { self.layout.as_ref().spacing() };
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
                        let spacing = unsafe { self.layout.as_ref().spacing() };
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
        unsafe {
            let margins = self.layout.contents_margins();
            layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
        }
    }
}

impl ControlInner for QtLinearLayout {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
        let orientation = self.orientation(member);
        
        let (mut x, mut y, spacing, pw, ph) = unsafe { 
            let margins = self.layout.contents_margins();
            let x = margins.left();
            let y = margins.top();
            let pw = utils::coord_to_size(cmp::max(0, pw as i32 - margins.left() - margins.right()));
            let ph = utils::coord_to_size(cmp::max(0, ph as i32 - margins.top() - margins.bottom()));
            let spacing = self.layout.as_ref().spacing();
            (x, y, spacing, pw, ph)
        };
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
    fn orientation(&self,_base: &MemberBase) -> layout::Orientation {
        box_direction_to_orientation(unsafe { (self.base.widget.as_ref().layout().as_ref().unwrap().dynamic_cast::<QBoxLayout>()).unwrap().direction() })
    }
    fn set_orientation(&mut self, _base: &mut MemberBase, orientation: layout::Orientation) {
        unsafe { self.base.widget.layout().dynamic_cast_mut::<QBoxLayout>().set_direction(orientation_to_box_direction(orientation)); }
        self.base.invalidate();
    }
}

impl ContainerInner for QtLinearLayout {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for child in self.children.as_mut_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member_mut().id() == *id {
                        return Some(child.as_mut());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member_mut().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_mut());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_mut(arg.clone());
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        for child in self.children.as_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member().id() == *id {
                        return Some(child.as_ref());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_ref());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container() {
                let ret = c.find_control(arg.clone());
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
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, mut child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let old = if index < self.children.len() {
            mem::swap(&mut child, &mut self.children[index]);
            unsafe {
                (self.base.widget.layout().dynamic_cast_mut::<QBoxLayout>()).remove_widget(MutPtr::from_raw(common::cast_control_to_qwidget_mut(child.as_mut())));
            }
            Some(child)
        } else {
            self.children.insert(index, child);
            None
        };
        unsafe {
            (self.base.widget.layout().dynamic_cast_mut::<QBoxLayout>()).insert_widget_2a(index as i32, MutPtr::from_raw(common::cast_control_to_qwidget_mut(self.children[index].as_mut()) as *mut QWidget));
        }
        
        if self.children.iter().find(|child| match self.orientation(base) {
                    layout::Orientation::Horizontal => child.layout_width() == layout::Size::MatchParent,
                    layout::Orientation::Vertical => child.layout_height() == layout::Size::MatchParent,
                }).is_none() {
            unsafe {
                let stretch = self.layout.item_at(self.layout.count()-1);
                if stretch.is_null() || (stretch.dynamic_cast::<QSpacerItem>()).is_null() {
                    self.layout.add_stretch_1a(0);
                }
            }
        } else {
            unsafe {
                let stretch = self.layout.item_at(self.layout.count()-1);
                if !stretch.is_null() && !(stretch.dynamic_cast::<QSpacerItem>()).is_null() {
                    self.layout.remove_item(stretch);
                }
            }
        }
        
        self.base.invalidate();
        old
    }
    fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>> {
        if index < self.children.len() {
            let mut item = self.children.remove(index);
            unsafe {
                (self.base.widget.layout().dynamic_cast_mut::<QBoxLayout>()).remove_widget(MutPtr::from_raw(common::cast_control_to_qwidget_mut(item.as_mut())));
            }
            if self.children.iter().find(|child| match self.orientation(base) {
                        layout::Orientation::Horizontal => child.layout_width() == layout::Size::MatchParent,
                        layout::Orientation::Vertical => child.layout_height() == layout::Size::MatchParent,
                    }).is_none() {
                unsafe {
                    let stretch = self.layout.item_at(self.layout.count()-1);
                    if stretch.is_null() || (stretch.dynamic_cast::<QSpacerItem>()).is_null() {
                        self.layout.add_stretch_1a(0);
                    }
                }
            } else {
                unsafe {
                    let stretch = self.layout.item_at(self.layout.count()-1);
                    if !stretch.is_null() && !(stretch.dynamic_cast::<QSpacerItem>()).is_null() {
                        self.layout.remove_item(stretch);
                    }
                }
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

fn box_direction_to_orientation(a: QDirection) -> layout::Orientation {
    match a {
        QDirection::TopToBottom => layout::Orientation::Vertical,
        QDirection::LeftToRight => layout::Orientation::Horizontal,
        _ => unreachable!(),
    }
}
fn orientation_to_box_direction(a: layout::Orientation) -> QDirection {
    match a {
        layout::Orientation::Horizontal => QDirection::LeftToRight,
        layout::Orientation::Vertical => QDirection::TopToBottom,
    }
}

impl Spawnable for QtLinearLayout {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_orientation(layout::Orientation::Vertical).into_control()
    }
}

fn event_handler<O: controls::LinearLayout>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<LinearLayout>(object) {
                let size = unsafe { 
                    let size = MutPtr::from_raw(event).static_downcast_mut::<QResizeEvent>();
                    let size = (
                    	utils::coord_to_size(size.size().width()), 
                    	utils::coord_to_size(size.size().height())
                    );
                    size
                };
                this.inner_mut().base.measured = size;
                this.call_on_size::<O>(size.0, size.1);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<LinearLayout>(object) {
                unsafe {
                    ptr::write(&mut ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.widget, common::MaybeCppBox::None);
                    ptr::write(&mut ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().layout, common::MaybeCppBox::None);
                }
            }
        }
        _ => {}
    }
    false
}
