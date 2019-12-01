use crate::common::{self, *};

use qt_core::list::ListCInt;
use qt_core::slots::SlotCInt;
use qt_widgets::box_layout::BoxLayout as QBoxLayout;
use qt_widgets::splitter::Splitter as QSplitter;

pub type Splitted = Member<Control<MultiContainer<QtSplitted>>>;

#[repr(C)]
pub struct QtSplitted {
    base: common::QtControlBase<Splitted, QSplitter>,
    splitter: f32,
    first: Box<dyn controls::Control>,
    second: Box<dyn controls::Control>,
    splitter_moved: SlotCInt<'static>,
}

impl SplittedInner for QtSplitted {
    fn with_content(mut first: Box<dyn controls::Control>, mut second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<Member<Control<MultiContainer<Self>>>> {
        let mut qsplitter = QSplitter::new(orientation_to_qorientation(orientation));
        unsafe {
            qsplitter.insert_widget(0, common::cast_control_to_qwidget_mut(first.as_mut()));
            qsplitter.insert_widget(1, common::cast_control_to_qwidget_mut(second.as_mut()));
        }
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                MultiContainer::with_inner(
                    QtSplitted {
                        base: common::QtControlBase::with_params(qsplitter, event_handler),
                        splitter: defaults::SPLITTED_POSITION,
                        first: first,
                        second: second,
                        splitter_moved: SlotCInt::new(move |_| {}),
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = ll.as_ref() as *const _ as u64;
            ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.set_children_collapsible(false);
            ll.as_inner_mut().as_inner_mut().as_inner_mut().splitter_moved.set(move |position| {
                splitter_moved(&mut *(ptr as *mut Splitted), position);
            });
            ll.as_inner().as_inner().as_inner().base.widget.signals().splitter_moved().connect(&ll.as_inner().as_inner().as_inner().splitter_moved);
            {
                let qo: &mut QObject = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
                qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
            }
        }
        //ll.as_inner_mut().as_inner_mut().as_inner_mut().update_children_orientation();
        ll
    }
    fn set_splitter(&mut self, member: &mut MemberBase, pos: f32) {
        let pos = pos % 1.0;
        self.splitter = pos;
        self.update_splitter(unsafe { utils::base_to_impl_mut::<Splitted>(member) }.as_inner_mut().base_mut());
    }
    fn splitter(&self) -> f32 {
        self.splitter
    }

    fn first(&self) -> &dyn controls::Control {
        self.first.as_ref()
    }
    fn second(&self) -> &dyn controls::Control {
        self.second.as_ref()
    }
    fn first_mut(&mut self) -> &mut dyn controls::Control {
        self.first.as_mut()
    }
    fn second_mut(&mut self) -> &mut dyn controls::Control {
        self.second.as_mut()
    }
}

impl QtSplitted {
    fn children_sizes(&self, control: &mut ControlBase) -> (u16, u16) {
        let (w, h) = control.measured;
        let o = self.layout_orientation();
        let margins = self.base.widget.contents_margins();
        let handle = self.base.widget.handle_width();
        let (target, start, end) = match o {
            layout::Orientation::Horizontal => (w, margins.left(), margins.right()),
            layout::Orientation::Vertical => (h, margins.top(), margins.bottom()),
        };
        (
            utils::coord_to_size((target as f32 * self.splitter) as i32 - start - (handle / 2)),
            utils::coord_to_size((target as f32 * (1.0 - self.splitter)) as i32 - end - (handle / 2)),
        )
    }
    fn update_splitter(&mut self, control: &mut ControlBase) {
        let (first, second) = self.children_sizes(control);
        let mut list = ListCInt::new(());
        list.append(&(first as i32));
        list.append(&(second as i32));
        self.base.widget.as_mut().set_sizes(&list);
        self.update_children_layout(control);
    }
    fn update_children_layout(&mut self, control: &mut ControlBase) {
        let orientation = self.layout_orientation();
        let (first_size, second_size) = self.children_sizes(control);
        let (width, height) = control.measured;
        let margins = self.base.widget.contents_margins();
        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
            match orientation {
                layout::Orientation::Horizontal => {
                    child.measure(cmp::max(0, *size) as u16, cmp::max(0, height as i32 - margins.top() - margins.bottom()) as u16);
                }
                layout::Orientation::Vertical => {
                    child.measure(cmp::max(0, width as i32 - margins.left() - margins.right()) as u16, cmp::max(0, *size) as u16);
                }
            }
        }
    }
}

impl Drop for QtSplitted {
    fn drop(&mut self) {
        if !self.base.widget.is_null() {
            let qo = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
            if let Some(self2) = common::cast_qobject_to_uimember_mut::<Splitted>(unsafe { &mut *qo }) {
                self.first.on_removed_from_container(self2);
                self.second.on_removed_from_container(self2);
            }
        }
    }
}

impl HasNativeIdInner for QtSplitted {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
    }
}
impl HasVisibilityInner for QtSplitted {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtSplitted {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for QtSplitted {}

impl Drawable for QtSplitted {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        self.update_children_layout(control);
        let orientation = self.layout_orientation();
        let old_size = control.measured;
        let (first, second) = self.children_sizes(control);
        let margins = self.base.widget.contents_margins();

        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        for (size, child) in [(first, self.first.as_mut()), (second, self.second.as_mut())].iter_mut() {
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    let (cw, _, _) = child.measure(cmp::max(0, *size) as u16, cmp::max(0, parent_height as i32 - margins.top() - margins.bottom()) as u16);
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    let (cw, _, _) = child.measure(cmp::max(0, parent_width as i32 - margins.left() - margins.right()) as u16, cmp::max(0, *size) as u16);
                                    w = cmp::max(w, cw);
                                }
                            }
                            w += match child.visibility() {
                                types::Visibility::Gone => 0,
                                _ => cmp::max(0, self.base.widget.as_ref().handle_width()) as u16,
                            };
                        }
                        measured = true;
                        w
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        let mut h = 0;
                        for (size, child) in [(first, self.first.as_mut()), (second, self.second.as_mut())].iter_mut() {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = match orientation {
                                    layout::Orientation::Horizontal => child.measure(cmp::max(0, *size) as u16, cmp::max(0, parent_height as i32 - margins.top() - margins.bottom()) as u16),
                                    layout::Orientation::Vertical => child.measure(cmp::max(0, parent_width as i32 - margins.left() - margins.right()) as u16, cmp::max(0, *size) as u16),
                                };
                                ch
                            };
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    h = cmp::max(h, ch);
                                }
                                layout::Orientation::Vertical => {
                                    h += ch;
                                }
                            }
                            h += match child.visibility() {
                                types::Visibility::Gone => 0,
                                _ => cmp::max(0, self.base.widget.as_ref().handle_width()) as u16,
                            };
                        }
                        h
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
impl HasLayoutInner for QtSplitted {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.update_splitter(unsafe { utils::base_to_impl_mut::<Splitted>(base) }.as_inner_mut().base_mut());
        self.base.invalidate();
    }
}
impl ControlInner for QtSplitted {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.update_splitter(control);
        self.base.dirty = false;
        self.draw(member, control);

        let self2: &mut Splitted = unsafe { utils::base_to_impl_mut(member) };
        let handle = self.base.widget.handle_width();
        let margins = self.base.widget.contents_margins();
        let (first_size, second_size) = self.children_sizes(control);

        match self.layout_orientation() {
            layout::Orientation::Horizontal => {
                let h = utils::coord_to_size(ph as i32 - margins.top() - margins.bottom());
                self.first.on_added_to_container(self2, margins.left() + 6, margins.top() + 6, first_size, h);
                self.second.on_added_to_container(self2, margins.left() + first_size as i32 + handle + 6, margins.top() + 6, second_size, h);
            }
            layout::Orientation::Vertical => {
                let w = utils::coord_to_size(pw as i32 - margins.left() - margins.right());
                self.first.on_added_to_container(self2, margins.left() + 6, margins.top() + 6, w, first_size);
                self.second.on_added_to_container(self2, margins.left() + 6, margins.top() + first_size as i32 + handle + 6, w, second_size);
            }
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let self2: &mut Splitted = unsafe { utils::base_to_impl_mut(member) };
        for child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
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

        fill_from_markup_base!(self, markup, registry, Splitted, [MEMBER_ID_LAYOUT_LINEAR, MEMBER_TYPE_LINEAR_LAYOUT]);
        fill_from_markup_children!(self, markup, registry);
    }
}

impl HasOrientationInner for QtSplitted {
    fn layout_orientation(&self) -> layout::Orientation {
        qorientation_to_orientation(self.base.widget.as_ref().orientation())
    }
    fn set_layout_orientation(&mut self, _base: &mut MemberBase, orientation: layout::Orientation) {
        self.base.widget.as_mut().set_orientation(orientation_to_qorientation(orientation));
        //self.update_children_orientation();
        self.base.invalidate();
    }
}

impl ContainerInner for QtSplitted {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first_mut());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second_mut());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first_mut());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second_mut());
                    }
                }
            }
        }

        let self2: &mut QtSplitted = unsafe { mem::transmute(self as *mut QtSplitted) }; // bck is stupid
        if let Some(c) = self.first_mut().is_container_mut() {
            let ret = c.find_control_mut(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self2.second_mut().is_container_mut() {
            let ret = c.find_control_mut(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first.as_ref());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second.as_ref());
                    }
                }
            }
        }
        if let Some(c) = self.first().is_container() {
            let ret = c.find_control(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self.second().is_container() {
            let ret = c.find_control(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
}

impl MultiContainerInner for QtSplitted {
    fn len(&self) -> usize {
        2
    }
    fn set_child_to(&mut self, _base: &mut MemberBase, index: usize, mut child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        use qt_widgets::frame::Frame as QFrame;

        let added = match index {
            0 => &mut self.first,
            1 => &mut self.second,
            _ => return None,
        };
        mem::swap(added, &mut child);
        unsafe {
            ((self.base.widget.as_mut().static_cast_mut() as &mut QFrame).layout().as_mut().unwrap().dynamic_cast_mut().unwrap() as &mut QBoxLayout).remove_widget(common::cast_control_to_qwidget_mut(child.as_mut()));
            self.base.widget.as_mut().insert_widget(index as i32, common::cast_control_to_qwidget_mut(added.as_mut()));
        }
        self.base.invalidate();
        Some(child)
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, _: usize) -> Option<Box<dyn controls::Control>> {
        None
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        match index {
            0 => Some(self.first()),
            1 => Some(self.second()),
            _ => None,
        }
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        match index {
            0 => Some(self.first_mut()),
            1 => Some(self.second_mut()),
            _ => None,
        }
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Splitted::with_orientation(layout::Orientation::Vertical).into_control()
}*/

fn splitter_moved(ll: &mut Splitted, position: i32) {
    use plygui_api::controls::{HasOrientation, HasSize};

    if position < 1 {
        return;
    }
    let orientation = ll.layout_orientation();
    let (width, height) = ll.size();
    let splitter = position as f32
        / match orientation {
            layout::Orientation::Vertical => {
                if height > 0 {
                    height as f32
                } else {
                    position as f32 * 2.0
                }
            }
            layout::Orientation::Horizontal => {
                if width > 0 {
                    width as f32
                } else {
                    position as f32 * 2.0
                }
            }
        };
    let (_, c, ll) = ll.as_base_parts_mut();
    ll.as_inner_mut().splitter = splitter;
    ll.as_inner_mut().update_children_layout(c);
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Splitted>(object) {
                use qt_core::cpp_utils::UnsafeStaticCast;
            	
                let size = unsafe { event.static_cast_mut() as &mut ResizeEvent };
                let size = (
                	utils::coord_to_size(size.size().width()), 
                	utils::coord_to_size(size.size().height())
                );
                this.as_inner_mut().base_mut().measured = size;
                this.call_on_size(size.0, size.1);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Splitted>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut()));
                }
            }
        }
        _ => {}
    }
    false
}
default_impls_as!(Splitted);
