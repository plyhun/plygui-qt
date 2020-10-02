use crate::common::{self, *};

use qt_widgets::QTreeWidget;
use qt_widgets::QTreeWidgetItem;

pub type Tree = AMember<AControl<AContainer<AAdapted<ATree<QtTree>>>>>;

struct TreeNode {
    node: adapter::Node,
    root: Box<dyn controls::Control>,
    widget: common::MaybeCppBox<QTreeWidgetItem>,
    branches: Vec<TreeNode>,
}

#[repr(C)]
pub struct QtTree {
    base: common::QtControlBase<Tree, QTreeWidget>,
    items: Vec<TreeNode>,
    h_left_clicked: (Option<callbacks::OnItemClick>, QBox<SlotNoArgs>),
}
impl ItemClickableInner for QtTree {
    fn item_click(&mut self, i: &[usize], item_view: &mut dyn controls::Control, _skip_callbacks: bool) {
        let this = common::cast_qobject_to_uimember_mut::<Tree>(&mut self.base.as_qwidget()).unwrap();
        if let Some(ref mut callback) = self.h_left_clicked.0 {
            (callback.as_mut())(this, i, item_view)
        }
    }
    fn on_item_click(&mut self, cb: Option<callbacks::OnItemClick>) {
        self.h_left_clicked.0 = cb;
    }
}
impl QtTree {
    fn add_item_inner(&mut self, base: &mut MemberBase, indexes: &[usize], node: &adapter::Node, y: &mut i32) {
        let (member, control, adapter, _) = unsafe { Tree::adapter_base_parts_mut(base) };
        let (pw, ph) = control.measured;
        let this: &mut Tree = unsafe { utils::base_to_impl_mut(member) };
        
        let mut item = adapter.adapter.spawn_item_view(indexes, this);
        
        let mut items = &mut self.items;
        let mut iter: Ptr<QTreeWidgetItem> = unsafe { Ptr::null() };
        for i in 0..indexes.len() {
            let index = indexes[i];
            let end = i+1 >= indexes.len();
            if end {
                items.insert(index, TreeNode {
                    node: node.clone(),
                    root: {
                        item.as_mut().map(|item| {
                                item.set_layout_width(layout::Size::WrapContent);
                                item.as_mut()
                            }).unwrap().on_added_to_container(this, 0, *y, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
                        item.take().unwrap()
                    },
                    widget: common::MaybeCppBox::Some(unsafe { QTreeWidgetItem::new() }),
                    branches: vec![]
                });
                let node = items.get_mut(index).unwrap();
	            let widget = unsafe { Ptr::from_raw(common::cast_control_to_qwidget_mut(node.root.as_mut())) };
	            
	            unsafe { 
	            	node.widget.set_child_indicator_policy(0);
	                node.widget.set_size_hint(0, &widget.size_hint());
	                if iter.is_null() {
		                self.base.widget.insert_top_level_item(i as i32, node.widget.as_ptr()); 
	                } else {
		                iter.insert_child(i as i32, node.widget.as_ptr());
	                }
	                
	                self.base.widget.set_item_widget(node.widget.as_ptr(), 0, widget);
	                widget.show();
	            }
	            return;
            } else {
                iter = unsafe {
                	if iter.is_null() {
		                self.base.widget.top_level_item(index as i32)
	                } else {
		                iter.child(index as i32)
	                }
                };
                items = &mut items[index].branches;
            }
        }
    }
    fn remove_item_inner(&mut self, base: &mut MemberBase, indexes: &[usize]) {
        let this: &mut Tree = unsafe { utils::base_to_impl_mut(base) };
        let mut items = &mut self.items;
        let mut iter: Ptr<QTreeWidgetItem> = unsafe { Ptr::null() };
        for i in 0..indexes.len() {
            let index = indexes[i];
            if index >= (items.len()-1) {
                let mut item = items.remove(index);
                item.root.on_removed_from_container(this);
                unsafe {
                	self.base.widget.remove_item_widget(item.widget.as_ptr(), 0); 
	                if iter.is_null() {
		                self.base.widget.take_top_level_item(i as i32); 
	                } else {
		                iter.remove_child(item.widget.as_ptr());
	                }
                }
            } else {
                unsafe {
                	iter = if iter.is_null() {
		                self.base.widget.top_level_item(index as i32)
	                } else {
		                iter.child(index as i32)
	                };
                }
                items = &mut items[index].branches;
            }
        }
    }
}

impl<O: controls::Tree> NewTreeInner<O> for QtTree {
    fn with_uninit(ptr: &mut mem::MaybeUninit<O>) -> Self {
        let mut ll = QtTree {
            base: common::QtControlBase::with_params(unsafe { QTreeWidget::new_0a() }, event_handler::<O>),
            items: Vec::new(),
            h_left_clicked: (None, unsafe { SlotNoArgs::new(NullPtr, move || {}) }), // dummy
        };
        unsafe {
            let ptr = ptr as *const _ as u64;
            let obj = ll.base.widget.static_upcast::<QObject>().as_mut_raw_ptr();
            ll.h_left_clicked.1 = SlotNoArgs::new(NullPtr, move || {
                let this = cast_qobject_to_uimember_mut::<Tree>(&mut *obj).unwrap();
                let clicked = this.inner().inner().inner().inner().inner().base.widget.current_item();
                let mut indexes = Vec::new();
                
                while {
	                let i = if clicked.parent().is_null() {
		                this.inner().inner().inner().inner().inner().base.widget.index_of_top_level_item(clicked)
	                } else {
		                clicked.parent().index_of_child(clicked)
	                };
	                indexes.push(i as usize);
	                !clicked.parent().is_null()
                } {}
                
                println!("clicked idx {:?}", indexes.as_slice());
                
                let this = cast_qobject_to_uimember_mut::<Tree>(&mut *obj).unwrap();
                if let Some(ref mut cb) = this.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().h_left_clicked.0 {
                    let this = cast_qobject_to_uimember_mut::<O>(&mut *obj).unwrap();
                    //let clicked = cast_qobject_to_base_mut(clicked.static_upcast::<QObject>().unwrap().as_ref()).unwrap();
                    //(cb.as_mut())(this, indexes.as_slice(), clicked.as_member_mut().is_control_mut().unwrap());
                }
            });
            ll.base.widget.item_clicked().connect(&ll.h_left_clicked.1);
            let qo = ll.base.widget.static_upcast::<QObject>();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        ll
    }
}
impl TreeInner for QtTree {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn controls::Tree> {
        let mut b: Box<mem::MaybeUninit<Tree>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        ATree::with_inner(
                            <Self as NewTreeInner<Tree>>::with_uninit(b.as_mut())
                        ),
                        adapter,
                        &mut b,
                    ),
                )
            ),
        );
        ab.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items = Vec::new();
        let mut bb = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let (member, _, adapter, tree) = unsafe { Tree::adapter_base_parts_mut(&mut bb.base) };

        let mut y = 0;
        adapter.adapter.for_each(&mut (|indexes, node| {
            tree.inner_mut().add_item_inner(member, indexes, node, &mut y);
        }));
        bb
    }
}
impl AdaptedInner for QtTree {
	fn on_item_change(&mut self, base: &mut MemberBase, change: adapter::Change) {
		let mut y = 0;
        {
            fn yadder(level: &[TreeNode], y: &mut i32) {
                for item in level {
                    let (_, yy) = item.root.size();
                    *y += yy as i32;
                    yadder(item.branches.as_slice(), y);
                }
            };
            yadder(self.items.as_slice(), &mut y);        
        }
        match change {
            adapter::Change::Added(at, ref node) => {
                self.add_item_inner(base, at, node, &mut y);
            },
            adapter::Change::Removed(at) => {
                self.remove_item_inner(base, at);
            },
            adapter::Change::Edited(_,_) => {
            },
        }
        self.base.invalidate();
	}
}
impl HasNativeIdInner for QtTree {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.base.widget.static_upcast::<QObject>().as_raw_ptr() } as *mut QObject)
    }
}
impl HasVisibilityInner for QtTree {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtTree {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtTree {}

impl Drawable for QtTree {
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

impl HasLayoutInner for QtTree {
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

impl ControlInner for QtTree {
    fn on_added_to_container(&mut self, member: &mut MemberBase, _: &mut ControlBase, _parent: &dyn controls::Container, _x: i32, _y: i32, pw: u16, ph: u16) {
        let self2: &mut Tree = unsafe { utils::base_to_impl_mut(member) };
        for node in self.items.iter_mut() {
            //child.on_added_to_container(self2, 0, 0, pw, ph);
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _parent: &dyn controls::Container) {
        let self2: &mut Tree = unsafe { utils::base_to_impl_mut(member) };
        for node in self.items.iter_mut() {
            //child.on_removed_from_container(self2);
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
        use plygui_api::markup::MEMBER_TYPE_tree;

        fill_from_markup_base!(self, markup, registry, Tree, [MEMBER_ID_layout_linear, MEMBER_TYPE_tree]);
        fill_from_markup_items!(self, markup, registry);
    }
}

impl ContainerInner for QtTree {
    fn find_control_mut<'a>(&'a mut self, arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
        fn find_control_inner_mut<'a>(vec: &'a mut [TreeNode], arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
            for child in vec {
                match arg {
                    types::FindBy::Id(id) => {
                        if child.root.as_member_mut().id() == id {
                            return Some(child.root.as_mut());
                        }
                    }
                    types::FindBy::Tag(tag) => {
                        if let Some(mytag) = child.root.as_member_mut().tag() {
                            if tag == mytag {
                                return Some(child.root.as_mut());
                            }
                        }
                    }
                }
                if let Some(c) = child.root.is_container_mut() {
                    let ret = c.find_control_mut(arg);
                    if ret.is_some() {
                        return ret;
                    }
                }
                let ret = find_control_inner_mut(child.branches.as_mut_slice(), arg);
                if ret.is_some() {
                    return ret;
                }
            }
            None
        }
        
        find_control_inner_mut(self.items.as_mut_slice(), arg)
    }
    fn find_control<'a>(&'a self, arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
        fn find_control_inner<'a>(vec: &'a [TreeNode], arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
            for child in vec {
                match arg {
                    types::FindBy::Id(id) => {
                        if child.root.as_member().id() == id {
                            return Some(child.root.as_ref());
                        }
                    }
                    types::FindBy::Tag(tag) => {
                        if let Some(mytag) = child.root.as_member().tag() {
                            if tag == mytag {
                                return Some(child.root.as_ref());
                            }
                        }
                    }
                }
                if let Some(c) = child.root.is_container() {
                    let ret = c.find_control(arg);
                    if ret.is_some() {
                        return ret;
                    }
                }
                let ret = find_control_inner(child.branches.as_slice(), arg);
                if ret.is_some() {
                    return ret;
                }
            }
            None
        }
        
        find_control_inner(self.items.as_slice(), arg)
    }
}
impl Spawnable for QtTree {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}
/*
impl Drop for QtTree {
	fn drop(&mut self) {
		for item in self.items {
            unsafe {
                ptr::write(&mut item.1, common::MaybeCppBox::None);
            }
    	}
	}
}
*/
fn event_handler<O: controls::Tree>(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Tree>(object) {
                let size = unsafe { 
                    let size = Ref::from_raw(event).unwrap().static_downcast::<QResizeEvent>();
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
            if let Some(ll) = cast_qobject_to_uimember_mut::<Tree>(object) {
            	for item in ll.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().items.as_mut_slice() {
	                unsafe {
	                    //ptr::write(&mut item.1, common::MaybeCppBox::None);
	                }
            	}
            }
        }
        _ => {}
    }
    false
}
