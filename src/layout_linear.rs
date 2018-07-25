use super::*;
use super::common::*;

use plygui_api::{layout, ids, types, controls};
use plygui_api::development::*;

use qt_widgets::frame::{Frame as QFrame};

use std::mem;
use std::os::raw::c_void;

const DEFAULT_PADDING: i32 = 0;

pub type LinearLayout = Member<Control<MultiContainer<QtLinearLayout>>>;

#[repr(C)]
pub struct QtLinearLayout {
    base: common::QtControlBase<LinearLayout>,
    orientation: layout::Orientation,
    gravity_horizontal: layout::Gravity,
    gravity_vertical: layout::Gravity,
    children: Vec<Box<controls::Control>>,
}

impl LinearLayoutInner for QtLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<LinearLayout> {
        use plygui_api::controls::HasLayout;
        
        let mut ll = Box::new(Member::with_inner(Control::with_inner(MultiContainer::with_inner(QtLinearLayout {
                     base: common::QtControlBase::with_params(
                     	unsafe {(&mut *QFrame::new().into_raw()).static_cast_mut() as &mut QWidget},
                     	event_handler,
                     ),
                     gravity_horizontal: Default::default(),
                    gravity_vertical: Default::default(),
                    orientation: orientation,
                     children: Vec::new(),
                 }, ()), ()), MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        unsafe {
        	let ptr = ll.as_ref() as *const _ as u64;
        	let qo: &mut QObject = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
        	qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        ll.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        ll
    }
}

impl MemberInner for QtLinearLayout {
    type Id = common::QtId;

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.base.invalidate()
    }

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.as_ref() as *const QWidget as *mut QWidget)
    }
}

impl Drawable for QtLinearLayout {
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some((x, y)) = self.base.coords {
    		let orientation = self.layout_orientation();
			let (lp,tp,_,_) = base.control.layout.padding.into();
	    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
	        self.base.widget.as_mut().move_((x as i32 + lm, y as i32 + tm));
			self.base.widget.as_mut().set_fixed_size(
				(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm)
			);
			let mut x = x + lp + lm;
	        let mut y = y + tp + tm;
	        for ref mut child in self.children.as_mut_slice() {
	            child.draw(Some((x, y)));
	            let (xx, yy) = child.size();
	            match orientation {
	                layout::Orientation::Horizontal => x += xx as i32,
	                layout::Orientation::Vertical => y += yy as i32,
	            }
	        }
		}
    }
    fn measure(&mut self, base: &mut MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	use std::cmp::max;
    	
    	let orientation = self.layout_orientation();
    	let old_size = self.base.measured_size;
    	let (lp,tp,rp,bp) = base.control.layout.padding.into();
    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
    	self.base.measured_size = match base.member.visibility {
        	types::Visibility::Gone => (0,0),
        	_ => {
        		let mut measured = false;
        		let w = match base.control.layout.width {
        			layout::Size::Exact(w) => w,
        			layout::Size::MatchParent => parent_width,
        			layout::Size::WrapContent => {
	        			let mut w = 0;
		                for ref mut child in self.children.as_mut_slice() {
		                    let (cw, _, _) = child.measure(
		                    	max(0, parent_width as i32 - lp - rp - lm - rm) as u16, 
		                    	max(0, parent_height as i32 - tp - bp - tm - bm) as u16
		                    );
		                    match orientation {
		                    	layout::Orientation::Horizontal => {
			                    	w += cw;
			                    },
		                    	layout::Orientation::Vertical => {
			                    	w = max(w, cw);
			                    },
		                    }
		                }
	        			measured = true;
	        			max(0, w as i32 + lm + rm + lp + rp) as u16
        			}
        		};
        		let h = match base.control.layout.height {
        			layout::Size::Exact(h) => h,
        			layout::Size::MatchParent => parent_height,
        			layout::Size::WrapContent => {
	        			let mut h = 0;
		                for ref mut child in self.children.as_mut_slice() {
		                    let ch = if measured {
		                    	child.size().1
		                    } else {
		                    	let (_, ch, _) = child.measure(
			                    	max(0, parent_width as i32 - lp - rp - lm - rm) as u16, 
			                    	max(0, parent_height as i32 - tp - bp - tm - bm) as u16
			                    );
		                    	ch
		                    };
		                    match orientation {
		                    	layout::Orientation::Horizontal => {
			                    	h = max(h, ch);
			                    },
		                    	layout::Orientation::Vertical => {
			                    	h += ch;
			                    },
		                    }
		                }
	        			max(0, h as i32 + tm + bm + tp + bp) as u16
        			}
        		};
        		(w, h)
        	}
        };
    	self.base.dirty = self.base.measured_size != old_size;
        (
            self.base.measured_size.0,
            self.base.measured_size.1,
            self.base.dirty,
        )
    }
    fn invalidate(&mut self, base: &mut MemberControlBase) {
        self.base.invalidate()
    }
}

impl HasLayoutInner for QtLinearLayout {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtLinearLayout {
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
        let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.base.dirty = false;
        self.draw(base, Some((x, y)));
        
        let selfptr = self as *mut _ as *mut c_void;
        let orientation = self.layout_orientation();
        let (lp,tp,_,_) = base.control.layout.padding.into();
    	let (lm,tm,_,_) = base.control.layout.margin.into();
        let mut x = x + lp + lm;
        let mut y = y + tp + tm;
        for ref mut child in self.children.as_mut_slice() {
            let self2: &mut LinearLayout = unsafe { mem::transmute(selfptr) };
            child.on_added_to_container(self2, x, y);
            let (xx, yy) = child.size();
            match orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
    }
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container) {
        let selfptr = self as *mut _ as *mut c_void;
        for mut child in self.children.drain(..) {
            let self2: &mut LinearLayout = unsafe { mem::transmute(selfptr) };
            child.on_removed_from_container(self2);
        }
    }

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut()
    }
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;
    	
    	fill_from_markup_base!(self, markup, registry, LinearLayout, [MEMBER_ID_LAYOUT_LINEAR, MEMBER_TYPE_LINEAR_LAYOUT]);
		fill_from_markup_children!(self, markup, registry);	
    }
}

impl HasOrientationInner for QtLinearLayout {
    fn layout_orientation(&self) -> layout::Orientation {
        self.orientation
    }
    fn set_layout_orientation(&mut self, base: &mut MemberBase, orientation: layout::Orientation) {
        self.orientation = orientation;
		self.base.invalidate();
    }
}

impl ContainerInner for QtLinearLayout {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
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
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
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

    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
    	(self.gravity_horizontal, self.gravity_vertical)
    }
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) {
    	if self.gravity_horizontal != w || self.gravity_vertical != h {
    		self.gravity_horizontal = w;
    		self.gravity_vertical = h;
    		self.invalidate(unsafe { mem::transmute(base) });
    	}
    }
}

impl MultiContainerInner for QtLinearLayout {
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<controls::Control>) -> Option<Box<controls::Control>> {
        //TODO need a way to swap old item with new
        self.children.insert(index, child);
        unsafe {
        	let base = common::cast_control_to_qwidget_mut(self.children[index].as_mut());						
        	base.set_parent(self.base.widget.as_mut_ptr());
        }
        if (index + 1) >= self.children.len() {
            return None;
        }
        Some(self.children.remove(index + 1))
    }
    fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<controls::Control>> {
        if index < self.children.len() {
        	let mut item = self.children.remove(index);
        	unsafe {
	        	let base = common::cast_control_to_qwidget_mut(item.as_mut());						
	        	base.set_parent(QWidget::new().into_raw());
	        }
	        Some(item)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
        self.children.get(index).map(|c| c.as_ref())
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
        //self.children.get_mut(index).map(|c| c.as_mut()) //the anonymous lifetime #1 does not necessarily outlive the static lifetime
        if let Some(c) = self.children.get_mut(index) {
        	Some(c.as_mut())
        } else {
        	None
        }
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	LinearLayout::with_orientation(layout::Orientation::Vertical).into_control()
}

impl_all_defaults!(LinearLayout);

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
	unsafe {
		match event.type_() {
			QEventType::Resize => {
			    if let Some(ll) = cast_qobject_to_uimember_mut::<LinearLayout>(object) {
			        use plygui_api::controls::Member;
					
					if ll.as_inner().as_inner().as_inner().base.dirty {
						ll.as_inner_mut().as_inner_mut().as_inner_mut().base.dirty = false;
						let (width,height) = ll.size();
						ll.call_on_resize(width, height);
					}
			    }
			},
			_ => {},
		} 
		false
	}
}

