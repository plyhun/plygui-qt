use crate::common::{self, *};

use qt_widgets::list_widget::{ListWidget as QListWidget};
use qt_widgets::list_widget_item::{ListWidgetItem};

pub type List = Member<Control<Adapter<QtList>>>;

#[repr(C)]
pub struct QtList {
    base: common::QtControlBase<List, QListWidget>,
    items: Vec<Box<dyn controls::Control>>,
}

impl ListInner for QtList {}

impl QtList {
    fn add_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let (member, control, adapter) = List::adapter_base_parts_mut(base);
        let (pw, ph) = control.measured;
        let this: &mut List = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(i, this);
        item.on_added_to_container(this, 0, 0, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
        self.items.insert(i, item);
        let widget = common::cast_control_to_qwidget_mut(self.items.get_mut(i).unwrap().as_mut());        
        
        let mut item = ListWidgetItem::new(());
        
        unsafe { 
            item.set_size_hint(&widget.size_hint());
            self.base.widget.insert_item_unsafe(i as i32, item.as_mut_ptr()); 
            self.base.widget.set_item_widget(item.as_mut_ptr(), widget);
            widget.show();
        }
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, i: usize) {
        let this: &mut List = unsafe { utils::base_to_impl_mut(base) };
        self.items.remove(i).on_removed_from_container(this); 
        
        let item = self.base.widget.item(i as i32);
        unsafe { self.base.widget.remove_item_widget(item); }
        self.items.remove(i);
    }
}

impl AdapterViewInner for QtList {
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
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<List> {
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                Adapter::with_inner(
                    QtList {
                        base: common::QtControlBase::with_params(QListWidget::new(), event_handler),
                        items: Vec::new(),
                    },
                    adapter,
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = ll.as_ref() as *const _ as u64;
            let qo: &mut QObject = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        ll
    }
}

impl Drop for QtList {
    fn drop(&mut self) {
        if !self.base.widget.is_null() {
            let qo = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
            if let Some(self2) = common::cast_qobject_to_uimember_mut::<List>(unsafe { &mut *qo }) {
                for mut child in self.items.drain(..) {
                    child.on_removed_from_container(self2);
                }
            }
            //self.layout = CppBox::default();
        }
    }
}

impl HasNativeIdInner for QtList {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
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
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for QtList {}

impl Drawable for QtList {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);

        let margins = self.base.widget.contents_margins();
        let spacing = self.base.widget.spacing();
        let x = margins.left();
        let mut y = margins.top();
        for child in self.items.as_mut_slice() {
            child.draw(Some((x, y)));
            let (_, yy) = child.size();
            y += yy as i32 + spacing;
        }
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
        let margins = self.base.widget.contents_margins();
        layout::BoundarySize::Distinct(margins.left(), margins.top(), margins.right(), margins.bottom())
    }
}

impl ControlInner for QtList {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
        let (member, _, adapter) = List::adapter_base_parts_mut(member);

        for i in 0..adapter.adapter.len() {
            self.add_item_inner(member, i);
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let self2: &mut List = unsafe { utils::base_to_impl_mut(member) };
        for child in self.items.iter_mut() {
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
        for child in self.items.as_mut_slice() {
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
        for child in self.items.as_slice() {
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

default_impls_as!(List);

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<List>(object) {
                use plygui_api::controls::HasSize;

                let (width, height) = ll.size();
                ll.call_on_size(width, height);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<List>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut()));
                }
            }
        }
        _ => {}
    }
    false
}
