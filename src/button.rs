use super::*;
use super::common::*;

use plygui_api::{layout, types, development, callbacks};
use plygui_api::traits::{UiControl, UiHasLayout, UiHasLabel, UiButton, UiMember, UiContainer, UiClickable};
use plygui_api::members::MEMBER_ID_BUTTON;

use qt_core::rect::{Rect as QRect};
use qt_core::slots::SlotNoArgs;
use qt_core::connection::Signal;
use qt_gui::font_metrics::{FontMetrics as QFontMetrics};
use qt_widgets::push_button::{PushButton as QPushButton};

use std::borrow::Cow;
use std::cmp::max;

const DEFAULT_PADDING: i32 = 10;

#[repr(C)]
pub struct Button {
    base: common::QtControlBase,

    h_left_clicked: (bool, SlotNoArgs<'static>),
    h_right_clicked: Option<callbacks::Click>,
}

impl Button {
    pub fn new(label: &str) -> Box<Button> {
        let mut btn = Box::new(Button {
                     base: common::QtControlBase::with_params(
		                     	unsafe {(&mut *QPushButton::new(&QString::from_std_str(label)).into_raw()).static_cast_mut() as &mut QWidget},
		                     	invalidate_impl,
                             	development::UiMemberFunctions {
		                             fn_member_id: member_id,
								     fn_is_control: is_control,
								     fn_is_control_mut: is_control_mut,
								     fn_size: size,
	                             },
                             	event_handler,
                             ),
                     h_left_clicked: (false, SlotNoArgs::new(move ||{})),
                     h_right_clicked: None,
                 });
        unsafe {
        	let ptr = btn.as_ref() as *const _ as u64;
        	let qo = btn.base.widget.static_cast_mut() as &mut QObject;
        	qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        unsafe {
        	use qt_core::cpp_utils::DynamicCast;
        	
        	let qo: *mut QPushButton = btn.base.widget.dynamic_cast_mut().unwrap();
        	(&mut *qo).signals().released().connect(&btn.h_left_clicked.1);
        	//cast_qobject_mut::<QPushButton>(&mut *qo).signals().released().connect(&btn.h_left_clicked.1);
        }
        btn.set_layout_padding(layout::BoundarySize::AllTheSame(DEFAULT_PADDING).into());
        btn.set_label(label);
        btn
    }
}

impl UiHasLabel for Button {
	fn label<'a>(&'a self) -> Cow<'a, str> {
		let name = (&*self.base.widget.as_ref()).window_title().to_utf8();
        unsafe {
	      let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
	      Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
	    }
	}
    fn set_label(&mut self, label: &str) {
    	self.base.widget.set_window_title(&QString::from_std_str(label));        
    }
}

impl UiClickable for Button {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {
		self.h_left_clicked.0 = cb.is_some();
		if cb.is_some() {
			let mut cb = cb.unwrap();		
			let button: *mut Button = self;
			self.h_left_clicked.1.set(move || unsafe {
		        //let ptr = (&*btn).property(PROPERTY.as_ptr() as *const i8).to_u_long_long();
				//if ptr != 0 {
					//let button: &mut Button = ::std::mem::transmute(ptr);
					(cb.as_mut())(&mut *button);
				//}
	        });
		} else {
			self.h_left_clicked.1.clear();
		}
    }    
}

impl UiButton for Button {
    
    /*fn on_right_click(&mut self, cb: Option<Box<FnMut(&mut UiButton)>>) {
        self.h_right_clicked = cb;
    }*/
    
    fn as_control(&self) -> &UiControl {
    	self
    }
	fn as_control_mut(&mut self) -> &mut UiControl {
		self
	}
	fn as_clickable(&self) -> &UiClickable {
		self
	}
	fn as_clickable_mut(&mut self) -> &mut UiClickable {
		self
	}
	fn as_has_label(&self) -> &UiHasLabel {
		self
	}
	fn as_has_label_mut(&mut self) -> &mut UiHasLabel {
		self
	}
}

impl UiHasLayout for Button {
	fn layout_width(&self) -> layout::Size {
    	self.base.control_base.layout.width
    }
	fn layout_height(&self) -> layout::Size {
		self.base.control_base.layout.height
	}
	fn layout_gravity(&self) -> layout::Gravity {
		self.base.control_base.layout.gravity
	}
	fn layout_alignment(&self) -> layout::Alignment {
		self.base.control_base.layout.alignment
	}
	fn layout_padding(&self) -> layout::BoundarySize {
		self.base.control_base.layout.padding
	}
	fn layout_margin(&self) -> layout::BoundarySize {
		self.base.control_base.layout.margin
	}
	
	fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) {
		self.base.control_base.layout.padding = padding.into();
		self.base.invalidate();
	}
	fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) {
		self.base.control_base.layout.margin = margin.into();
		self.base.invalidate();
	} 
	fn set_layout_width(&mut self, width: layout::Size) {
		self.base.control_base.layout.width = width;
		self.base.invalidate();
	}
	fn set_layout_height(&mut self, height: layout::Size) {
		self.base.control_base.layout.height = height;
		self.base.invalidate();
	}
	fn set_layout_gravity(&mut self, gravity: layout::Gravity) {
		self.base.control_base.layout.gravity = gravity;
		self.base.invalidate();
	}
	fn set_layout_alignment(&mut self, alignment: layout::Alignment) {
		self.base.control_base.layout.alignment = alignment;
		self.base.invalidate();
	}   
	fn as_member(&self) -> &UiMember {
		self
	}
	fn as_member_mut(&mut self) -> &mut UiMember {
		self
	}
}

impl UiControl for Button {
    fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
        None
    }
    fn is_container(&self) -> Option<&UiContainer> {
        None
    }
    
    fn parent(&self) -> Option<&types::UiMemberBase> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&types::UiMemberBase> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> {
        self.base.root_mut()
    }
    fn on_added_to_container(&mut self, parent: &UiContainer, x: i32, y: i32) {
    	use plygui_api::development::UiDrawable;
    	
        let (pw, ph) = parent.size();
        self.measure(pw, ph);
        self.draw(Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &UiContainer) {
        unsafe { self.base.on_removed_from_container(); }
    }	
    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
    	use plygui_api::markup::MEMBER_TYPE_BUTTON;
    	
    	fill_from_markup_base!(self, markup, registry, Button, [MEMBER_ID_BUTTON, MEMBER_TYPE_BUTTON]);
    	fill_from_markup_label!(self, markup);
    	//fill_from_markup_callbacks!(self, markup, registry, ["on_left_click" => FnMut(&mut UiButton)]);
    	
    	if let Some(on_left_click) = markup.attributes.get("on_left_click") {
    		let callback: callbacks::Click = registry.pop_callback(on_left_click.as_attribute()).unwrap();
    		self.on_left_click(Some(callback));
    	}
    }
    fn as_has_layout(&self) -> &UiHasLayout {
    	self
    }
	fn as_has_layout_mut(&mut self) -> &mut UiHasLayout {
		self
	}
}

impl UiMember for Button {
    fn set_visibility(&mut self, visibility: types::Visibility) {
        self.base.set_visibility(visibility);
    }
    fn visibility(&self) -> types::Visibility {
        self.base.visibility()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    fn on_resize(&mut self, handler: Option<callbacks::Resize>) {
        self.base.h_resize = handler;
    }
	
    unsafe fn native_id(&self) -> usize {
        self.base.widget.win_id() as usize
    }
    fn is_control(&self) -> Option<&UiControl> {
    	Some(self)
    }
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
    	Some(self)
    } 
    fn as_base(&self) -> &types::UiMemberBase {
    	self.base.control_base.member_base.as_ref()
    }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase {
    	self.base.control_base.member_base.as_mut()
    }
}

impl development::UiDrawable for Button {
	fn draw(&mut self, coords: Option<(i32, i32)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
    	if let Some(coords) = self.base.coords {
			//let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
			let (lm,tm,_,_) = self.base.control_base.layout.margin.into();
			self.base.widget.as_mut().move_((coords.0 as i32 + lm, coords.1 as i32 + tm));
			self.base.widget.as_mut().set_fixed_size(
				(self.base.measured_size.0 as i32, self.base.measured_size.1 as i32)
			);
		}
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
    	let (lp,tp,rp,bp) = self.base.control_base.layout.padding.into();
    	let (lm,tm,rm,bm) = self.base.control_base.layout.margin.into();
    	
    	let font = self.base.widget.as_ref().font();
    	
        self.base.measured_size = match self.visibility() {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut label_size = QRect::new((0,0,0,0));
                let w = match self.base.control_base.layout.width {
                    layout::Size::MatchParent => parent_width as i32 - lm - rm,
                    layout::Size::Exact(w) => w as i32 - lm - rm,
                    layout::Size::WrapContent => {
                        if label_size.width() < 1 {
                        	let mut fm = QFontMetrics::new(font);
                        	label_size = fm.bounding_rect(&(&*self.base.widget.as_ref()).window_title());							
                        }
                        label_size.width() as i32 + lp + rp
                    } 
                };
                let h = match self.base.control_base.layout.height {
                    layout::Size::MatchParent => parent_height as i32 - tm - bm,
                    layout::Size::Exact(h) => h as i32 - tm - bm,
                    layout::Size::WrapContent => {
                        if label_size.height() < 1 {
                            let mut fm = QFontMetrics::new(font);
                        	label_size = fm.bounding_rect(&(&*self.base.widget.as_ref()).window_title());	
                        }
                        label_size.height() as i32 + tp + bp
                    } 
                };
                (max(0,w) as u16, max(0,h) as u16)
            },
        };
        (max(0, self.base.measured_size.0 as i32 + lm + rm) as u16, max(0, self.base.measured_size.1 as i32 + tm + bm) as u16, self.base.measured_size != old_size)
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<UiControl> {
	Button::new("")
}

impl_invalidate!(Button);
impl_is_control!(Button);
impl_size!(Button);
impl_member_id!(MEMBER_ID_BUTTON);

fn event_handler(object: &mut QObject, event: &QEvent) -> bool {
	unsafe {
		match event.type_() {
			QEventType::Resize => {
				let ptr = object as *mut QObject;
				if let Some(button) = cast_qobject_to_uimember_mut::<Button>(object) {
					let (width,height) = button.size();
					if let Some(ref mut cb) = button.base.h_resize {
		                let w2: &mut Button = ::std::mem::transmute(ptr);
		                (cb.as_mut())(w2, width, height);
		            }
				}
			},
			_ => {},
		} 
		false
	}
}