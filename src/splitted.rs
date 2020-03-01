use crate::common::{self, *};

use qt_core::QListOfInt;
use qt_core::SlotOfInt;
use qt_widgets::QBoxLayout;
use qt_widgets::QSplitter;

pub type Splitted = AMember<AControl<AContainer<AMultiContainer<ASplitted<QtSplitted>>>>>;

#[repr(C)]
pub struct QtSplitted {
    base: common::QtControlBase<Splitted, QSplitter>,
    splitter: f32,
    first: Box<dyn controls::Control>,
    second: Box<dyn controls::Control>,
    splitter_moved: SlotOfInt<'static>,
}

impl<O: controls::Splitted> NewSplittedInner<O> for QtSplitted {
    fn with_uninit_params(ptr: &mut mem::MaybeUninit<O>, mut first: Box<dyn controls::Control>, mut second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Self {
        let mut qsplitter = unsafe { QSplitter::from_orientation(orientation_to_qorientation(orientation)) };
        unsafe {
            qsplitter.insert_widget(0, MutPtr::from_raw(common::cast_control_to_qwidget_mut(first.as_mut())));
            qsplitter.insert_widget(1, MutPtr::from_raw(common::cast_control_to_qwidget_mut(second.as_mut())));
        }
        let mut ll = QtSplitted {
            base: common::QtControlBase::with_params(qsplitter, event_handler::<O>),
            splitter: defaults::SPLITTED_POSITION,
            first: first,
            second: second,
            splitter_moved: SlotOfInt::new(move |_| {}),
        };
        unsafe {
            let ptr = ptr as *const _ as u64;
            ll.base.widget.set_children_collapsible(false);
            ll.splitter_moved.set(move |position| {
                splitter_moved(&mut *(ptr as *mut Splitted), position);
            });
            ll.base.widget.splitter_moved().connect(&ll.splitter_moved);
            {
                let mut qo = ll.base.widget.static_upcast_mut::<QObject>();
                qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
            }
        }
        //ll.inner_mut().inner_mut().inner_mut().update_children_orientation();
        ll
    }
}
impl SplittedInner for QtSplitted {
    fn with_content(first: Box<dyn controls::Control>, second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<dyn controls::Splitted> {
        let mut b: Box<mem::MaybeUninit<Splitted>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AMultiContainer::with_inner(
                        ASplitted::with_inner(
                            <Self as NewSplittedInner<Splitted>>::with_uninit_params(b.as_mut(), first, second, orientation)
                        )
                    ),
                )
            ),
        );
        unsafe {
            b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
    fn set_splitter(&mut self, member: &mut MemberBase, pos: f32) {
        let pos = pos % 1.0;
        self.splitter = pos;
        let this = unsafe { utils::base_to_impl_mut::<Splitted>(member) };
        let (m, c, _) = Splitted::as_control_parts_mut(this);
        self.update_splitter(m, c);
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
    fn children_sizes(&self, member: &mut MemberBase, control: &mut ControlBase) -> (u16, u16) {
        let (w, h) = control.measured;
        let o = self.orientation(member);
        let (target, start, end, handle) = unsafe {
            let handle = self.base.widget.handle_width();
            let margins = self.base.widget.contents_margins();
            match o {
                layout::Orientation::Horizontal => (w, margins.left(), margins.right(), handle),
                layout::Orientation::Vertical => (h, margins.top(), margins.bottom(), handle),
            }
        };
        (
            utils::coord_to_size((target as f32 * self.splitter) as i32 - start - (handle / 2)),
            utils::coord_to_size((target as f32 * (1.0 - self.splitter)) as i32 - end - (handle / 2)),
        )
    }
    fn update_splitter(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        let (first, second) = self.children_sizes(member, control);
        unsafe {
            let mut list = QListOfInt::new();
            list.append_int(Ref::from_raw_ref(&(first as i32)));
            list.append_int(Ref::from_raw_ref(&(second as i32)));
            self.base.widget.set_sizes(&list);
        }
        self.update_children_layout(member, control);
    }
    fn update_children_layout(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        let orientation = self.orientation(member);
        let (first_size, second_size) = self.children_sizes(member, control);
        let (width, height) = control.measured;
        let margins = unsafe { self.base.widget.contents_margins() };
        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
            match orientation {
                layout::Orientation::Horizontal => {
                    child.measure(cmp::max(0, *size) as u16, unsafe { cmp::max(0, height as i32 - margins.top() - margins.bottom()) as u16 });
                }
                layout::Orientation::Vertical => {
                    child.measure(unsafe { cmp::max(0, width as i32 - margins.left() - margins.right()) as u16 }, cmp::max(0, *size) as u16);
                }
            }
        }
    }
}

impl HasNativeIdInner for QtSplitted {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.base.widget.static_upcast::<QObject>() }.as_raw_ptr() as *mut QObject)
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
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtSplitted {}

impl Drawable for QtSplitted {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        self.update_children_layout(member, control);
        let orientation = self.orientation(member);
        let old_size = control.measured;
        let (first, second) = self.children_sizes(member, control);
        let margins = unsafe { self.base.widget.contents_margins() };

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
                                    let (cw, _, _) = child.measure(cmp::max(0, *size) as u16, unsafe { cmp::max(0, parent_height as i32 - margins.top() - margins.bottom()) as u16 });
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    let (cw, _, _) = child.measure(unsafe { cmp::max(0, parent_width as i32 - margins.left() - margins.right()) as u16 }, cmp::max(0, *size) as u16);
                                    w = cmp::max(w, cw);
                                }
                            }
                            w += match child.visibility() {
                                types::Visibility::Gone => 0,
                                _ => cmp::max(0, unsafe { self.base.widget.as_ref().handle_width() }) as u16,
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
                                    layout::Orientation::Horizontal => child.measure(cmp::max(0, *size) as u16, unsafe { cmp::max(0, parent_height as i32 - margins.top() - margins.bottom()) as u16 }),
                                    layout::Orientation::Vertical => child.measure(unsafe { cmp::max(0, parent_width as i32 - margins.left() - margins.right()) as u16 }, cmp::max(0, *size) as u16),
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
                                _ => cmp::max(0, unsafe { self.base.widget.as_ref().handle_width() }) as u16,
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
        let this = unsafe { utils::base_to_impl_mut::<Splitted>(base) };
        let (m, c, _) = Splitted::as_control_parts_mut(this);
        self.update_splitter(m, c);
        self.base.invalidate();
    }
}
impl ControlInner for QtSplitted {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.update_splitter(member, control);
        self.base.dirty = false;
        self.draw(member, control);

        let (first_size, second_size) = self.children_sizes(member, control);

        unsafe {
            let handle = self.base.widget.handle_width();
            let margins = self.base.widget.contents_margins();
            match self.orientation(member) {
                layout::Orientation::Horizontal => {
                    let h = utils::coord_to_size(ph as i32 - margins.top() - margins.bottom());
                    let self2: &mut Splitted = utils::base_to_impl_mut(member);
                    self.first.on_added_to_container(self2, margins.left() + 6, margins.top() + 6, first_size, h);
                    self.second.on_added_to_container(self2, margins.left() + first_size as i32 + handle + 6, margins.top() + 6, second_size, h);
                }
                layout::Orientation::Vertical => {
                    let w = utils::coord_to_size(pw as i32 - margins.left() - margins.right());
                    let self2: &mut Splitted = utils::base_to_impl_mut(member);
                    self.first.on_added_to_container(self2, margins.left() + 6, margins.top() + 6, w, first_size);
                    self.second.on_added_to_container(self2, margins.left() + 6, margins.top() + first_size as i32 + handle + 6, w, second_size);
                }
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
    fn orientation(&self, _: &MemberBase) -> layout::Orientation {
        qorientation_to_orientation(unsafe { self.base.widget.as_ref().orientation() })
    }
    fn set_orientation(&mut self, _base: &mut MemberBase, orientation: layout::Orientation) {
        unsafe { self.base.widget.set_orientation(orientation_to_qorientation(orientation)) };
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
        use qt_widgets::QFrame;

        let added = match index {
            0 => &mut self.first,
            1 => &mut self.second,
            _ => return None,
        };
        mem::swap(added, &mut child);
        unsafe {
            ((self.base.widget.static_upcast_mut::<QFrame>()).layout().static_downcast_mut::<QBoxLayout>()).remove_widget(MutPtr::from_raw(common::cast_control_to_qwidget_mut(child.as_mut())));
            self.base.widget.insert_widget(index as i32, MutPtr::from_raw(common::cast_control_to_qwidget_mut(added.as_mut())));
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

impl Spawnable for QtSplitted {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_content(super::text::Text::spawn(), super::text::Text::spawn(), layout::Orientation::Vertical).into_control()
    }
}

fn splitter_moved(ll: &mut Splitted, position: i32) {
    use plygui_api::controls::{HasOrientation, HasSize};

    if position < 1 {
        return;
    }
    let orientation = HasOrientation::orientation(ll);
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
    let (m, c, ll) = ll.as_control_parts_mut();
    ll.inner_mut().inner_mut().inner_mut().splitter = splitter;
    ll.inner_mut().inner_mut().inner_mut().update_children_layout(m, c);
}

fn event_handler<O: controls::Splitted>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Splitted>(object) {
                let size = unsafe { 
                    let size = &mut MutRef::from_raw_ref(event).static_downcast_mut::<QResizeEvent>();
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
            if let Some(ll) = cast_qobject_to_uimember_mut::<Splitted>(object) {
                unsafe {
                    ptr::write(&mut ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.widget, common::MaybeCppBox::None);
                }
            }
        }
        _ => {}
    }
    false
}
