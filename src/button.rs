use super::*;
use super::common::*;

use qt_core::rect::{Rect as QRect};
use qt_core::slots::SlotNoArgs;
use qt_core::connection::Signal;
use qt_gui::font_metrics::{FontMetrics as QFontMetrics};
use qt_widgets::push_button::{PushButton as QPushButton};

use plygui_api::{layout, types, callbacks, controls};
use plygui_api::development::*;

use std::borrow::Cow;
use std::cmp::max;

const DEFAULT_PADDING: i32 = 6;

pub type Button = Member<Control<QtButton>>;

#[repr(C)]
pub struct QtButton {
    base: common::QtControlBase<Button>,

    h_left_clicked: (bool, SlotNoArgs<'static>),
}

impl HasLabelInner for QtButton {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		let name = (self.base.widget.as_ref().dynamic_cast().unwrap() as &QPushButton).text().to_utf8();
        unsafe {
	      let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
	      Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
	    }
	}
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
    	(self.base.widget.as_mut().dynamic_cast_mut().unwrap() as &mut QPushButton).set_text(&QString::from_std_str(label));        
    }
}

impl ClickableInner for QtButton {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {
		self.h_left_clicked.0 = cb.is_some();
		if cb.is_some() {
			let mut cb = cb.unwrap();
			let ptr = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
			self.h_left_clicked.1.set(move || unsafe {
			    let button = cast_qobject_to_uimember_mut::<Button>(&mut *ptr).unwrap();
		        //let ptr = (&*btn).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
				//if ptr != 0 {
					//let button: &mut Button = ::std::mem::transmute(ptr);
					(cb.as_mut())(button);
				//}
	        });
		} else {
			self.h_left_clicked.1.clear();
		}
    }    
}

impl ButtonInner for QtButton {
    fn with_label(label: &str) -> Box<Button> {
        use plygui_api::controls::{HasLabel, HasLayout};
        
        let mut btn = Box::new(Member::with_inner(Control::with_inner(QtButton {
                     base: common::QtControlBase::with_params(
		                     	unsafe {(&mut *QPushButton::new(&QString::from_std_str(label)).into_raw()).static_cast_mut() as &mut QWidget},
		                     	event_handler,
                             ),
                     h_left_clicked: (false, SlotNoArgs::new(move ||{})),
                 }, ()), MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut)));
        unsafe {
        	let ptr = btn.as_ref() as *const _ as u64;
        	let qo: &mut QObject = btn.as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
        	qo.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        unsafe {
        	use qt_core::cpp_utils::DynamicCast;
        	
        	let qo: *mut QPushButton = btn.as_inner_mut().as_inner_mut().base.widget.dynamic_cast_mut().unwrap();
        	(&mut *qo).signals().released().connect(&btn.as_inner_mut().as_inner_mut().h_left_clicked.1);
        }
        btn.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        btn.set_label(label);
        btn
    }
}

impl HasLayoutInner for QtButton {
	fn on_layout_changed(&mut self, base: &mut MemberBase) {
	    self.base.invalidate();
	}
}

impl ControlInner for QtButton {
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
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
    	let (pw, ph) = parent.draw_area_size();
        self.measure(base, pw, ph);
        self.base.dirty = false;
        self.draw(base, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &mut MemberControlBase, _: &controls::Container) {}	
    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_BUTTON;
    	
    	fill_from_markup_base!(self, markup, registry, Button, [MEMBER_ID_BUTTON, MEMBER_TYPE_BUTTON]);
    	fill_from_markup_label!(self, markup);
    	fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut controls::Button)]);
    }
}

impl MemberInner for QtButton {
    type Id = common::QtId;

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.base.invalidate()
    }

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.as_ref() as *const _ as *mut QWidget)
    }
}

impl Drawable for QtButton {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some(coords) = self.base.coords {
			let (lm,tm,rm,bm) = base.control.layout.margin.into();
	        self.base.widget.as_mut().move_((coords.0 as i32 + lm, coords.1 as i32 + tm));
			self.base.widget.as_mut().set_fixed_size(
				(self.base.measured_size.0 as i32 - lm - rm, self.base.measured_size.1 as i32 - rm - bm)
			);
		}
    }
    fn measure(&mut self, base: &mut MemberControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
    	self.base.measured_size = match base.member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let (lp,tp,rp,bp) = base.control.layout.padding.into();
		    	let (lm,tm,rm,bm) = base.control.layout.margin.into();
		    	    	
		    	let font = self.base.widget.as_ref().font();
		    	
		        let mut label_size = QRect::new((0,0,0,0));
                let w = match base.control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.width() < 1 {
                        	let mut fm = QFontMetrics::new(font);
                        	label_size = fm.bounding_rect(&(self.base.widget.as_ref().dynamic_cast().unwrap() as &QPushButton).text());							
                        }
                        label_size.width() + lp + rp + lm + rm + 16
                    } 
                };
                let h = match base.control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.height() < 1 {
                            let mut fm = QFontMetrics::new(font);
                        	label_size = fm.bounding_rect(&(self.base.widget.as_ref().dynamic_cast().unwrap() as &QPushButton).text());	
                        }
                        label_size.height() + tp + bp + tm + bm
                    } 
                };
                (max(0, w) as u16, max(0, h) as u16)
            },
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

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	Button::with_label("").into_control()
}

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
	unsafe {
		match event.type_() {
			QEventType::Resize => {
				if let Some(ll) = cast_qobject_to_uimember_mut::<Button>(object) {
			        use plygui_api::controls::Member;
					
					if ll.as_inner().as_inner().base.dirty {
						ll.as_inner_mut().as_inner_mut().base.dirty = false;
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
impl_all_defaults!(Button);
