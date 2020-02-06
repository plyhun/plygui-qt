use crate::common::{self, *};

use qt_widgets::QListWidget;
use qt_widgets::QListWidgetItem;

pub type List = AMember<AControl<AContainer<AAdapted<AList<QtList>>>>>;

#[repr(C)]
pub struct QtList {
    base: common::QtControlBase<List, QListWidget>,
    items: Vec<(Box<dyn controls::Control>, CppBox<QListWidgetItem>)>,
    h_left_clicked: (Option<callbacks::OnItemClick>, Slot<'static>),
}
impl ItemClickableInner for QtList {
    fn item_click(&mut self, i: usize, item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let this = common::cast_qobject_to_uimember_mut::<List>(&mut self.base.as_qwidget_mut()).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked.0 {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked.0 = cb;
    }
}
impl QtList {
    fn add_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let (member, control, adapter, _) = unsafe { List::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let this: &mut List = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(i, this);
        
        item.on_added_to_container(this, 0, 0, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
        self.items.insert(i, (item, unsafe { QListWidgetItem::new() }));
        let (item, witem) = self.items.get_mut(i).unwrap();
        let mut widget = unsafe { MutPtr::from_raw(common::cast_control_to_qwidget_mut(item.as_mut())) };        
        
        unsafe { 
            witem.set_size_hint(&widget.size_hint());
            self.base.widget.insert_item_int_q_list_widget_item(i as i32, witem.as_mut_ptr()); 
            self.base.widget.set_item_widget(witem.as_mut_ptr(), widget);
            widget.show();
        }
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let this: &mut List = unsafe { utils::base_to_impl_mut(base) };
        self.items.remove(i).0.on_removed_from_container(this); 
        
        unsafe { 
            let item = self.base.widget.item(i as i32);
            self.base.widget.remove_item_widget(item); 
        }
    }
}

impl<O: controls::List> NewListInner<O> for QtList {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let mut ll = QtList {
            base: common::QtControlBase::with_params(unsafe { QListWidget::new_0a() }, event_handler::<O>),
            items: Vec::new(),
            h_left_clicked: (None, Slot::new(move || {})), // dummy
        };
        unsafe {
            let ptr = ptr as *const _ as u64;
            let obj = ll.base.widget.static_upcast_mut::<QObject>().as_mut_raw_ptr();
            ll.h_left_clicked.1 = Slot::new(move || {
                let this = cast_qobject_to_uimember_mut::<List>(&mut *obj).unwrap();
                let clicked = this.inner().inner().inner().inner().inner().base.widget.current_item();
                let i = this.inner().inner().inner().inner().inner().base.widget.row(clicked);
                let (ref mut clicked,_) = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items.get_mut(i as usize).unwrap();
                let this = cast_qobject_to_uimember_mut::<List>(&mut *obj).unwrap();
                if let Some(ref mut cb) = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked.0 {
                    let this = cast_qobject_to_uimember_mut::<O>(&mut *obj).unwrap();
                    (cb.as_mut())(this, i as usize, clicked.as_mut());
                }
            });
            ll.base.widget.item_clicked().connect(&ll.h_left_clicked.1);
            let mut qo = ll.base.widget.static_upcast_mut::<QObject>();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        ll
    }
}
impl ListInner for QtList {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn controls::List> {
        let len = adapter.len();
        let mut b: Box<mem::MaybeUninit<List>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        AList::with_inner(
                            <Self as NewListInner<List>>::with_uninit(b.as_mut())
                        ),
                        adapter,
                        &mut b,
                    ),
                )
            ),
        );
        ab.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items = Vec::with_capacity(len);
        let mut bb = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let (member, _, adapter, list) = unsafe { List::adapter_base_parts_mut(&mut bb.base) };

        for i in 0..adapter.adapter.len() {
            list.inner_mut().add_item_inner(member, i);
        }
        bb
    }
}
impl AdaptedInner for QtList {
	fn on_item_change(&mut self, base: &mut MemberBase, change: types::Change) {
		match change {
            types::Change::Added(at) => {
                self.add_item_inner(base, at);
            },
            types::Change::Removed(at) => {
                self.remove_item_inner(base, at);
            },
            types::Change::Edited(_) => {
            },
        }
        self.base.invalidate();
	}
}
impl HasNativeIdInner for QtList {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl HasVisibilityInner for QtList {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtList {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtList {}

impl Drawable for QtList {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);

        /*let margins = self.base.widget.contents_margins();
        let spacing = self.base.widget.spacing();
        let x = margins.left();
        let mut y = margins.top();
        for child in self.items.as_mut_slice() {
            child.draw(Some((x, y)));
            let (_, yy) = child.size();
            y += yy as i32 + spacing;
        }*/
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

impl HasLayoutInner for QtList {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        unsafe {
            let margins = self.base.widget.contents_margins();
            layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
        }
    }
}

impl Drop for QtList {
    fn drop(&mut self) {
        self.items.clear();
    }
}

impl ControlInner for QtList {
    fn on_added_to_container(&mut self, member: &mut MemberBase, _: &mut ControlBase, _parent: &dyn controls::Container, _x: i32, _y: i32, pw: u16, ph: u16) {
        let self2: &mut List = unsafe { utils::base_to_impl_mut(member) };
        for (child,_) in self.items.iter_mut() {
            child.on_added_to_container(self2, 0, 0, pw, ph);
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let self2: &mut List = unsafe { utils::base_to_impl_mut(member) };
        for (child,_) in self.items.iter_mut() {
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
        use plygui_api::markup::MEMBER_TYPE_list;

        fill_from_markup_base!(self, markup, registry, List, [MEMBER_ID_layout_linear, MEMBER_TYPE_list]);
        fill_from_markup_items!(self, markup, registry);
    }
}

impl ContainerInner for QtList {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for (child,_) in self.items.as_mut_slice() {
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
        for (child,_) in self.items.as_slice() {
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
impl Spawnable for QtList {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}

fn event_handler<O: controls::List>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<List>(object) {
                let size = unsafe { 
                    let size = MutRef::from_raw_ref(event).static_downcast_mut::<QResizeEvent>();
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
            if let Some(ll) = cast_qobject_to_uimember_mut::<List>(object) {
                unsafe {
                    ptr::write(&mut ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.widget, common::MaybeCppBox::None);
                }
            }
        }
        _ => {}
    }
    false
}
