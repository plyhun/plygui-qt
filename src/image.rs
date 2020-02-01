use crate::common::{self, *};

use qt_core::{AlignmentFlag, AspectRatioMode};
use qt_core::QRect;
use qt_widgets::QLabel;

pub type Image = Member<Control<QtImage>>;

#[repr(C)]
pub struct QtImage {
    base: QtControlBase<Image, QLabel>,

    scale: types::ImageScalePolicy,
    pixmap: CppBox<QPixmap>,
    content: image::DynamicImage,
}

impl ImageInner for QtImage {
    fn with_content(content: image::DynamicImage) -> Box<dyn controls::Image> {
        let mut i = Box::new(Member::with_inner(
            Control::with_inner(
                QtImage {
                    base: QtControlBase::with_params(unsafe { QLabel::new() }, event_handler),
                    scale: types::ImageScalePolicy::FitCenter,
                    pixmap: unsafe { CppBox::new(MutPtr::null()).unwrap() },
                    content: content,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        unsafe {
            let ptr = i.as_ref() as *const _ as u64;
            let mut qo = i.as_inner_mut().as_inner_mut().base.widget.static_upcast_mut::<QObject>().as_mut_ptr();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
            i.as_inner_mut().as_inner_mut().base.widget.set_alignment(AlignmentFlag::AlignCenter.into());
        }
        i
    }
    fn set_scale(&mut self, _: &mut MemberBase, policy: types::ImageScalePolicy) {
        if self.scale != policy {
            self.scale = policy;
            self.base.invalidate();
        }
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.scale
    }
}

impl QtImage {
    fn update_image(&mut self, control: &mut ControlBase) {
        use image::GenericImageView;
    
        let (w, h) = self.content.dimensions();
        let (iw, ih) = control.measured;
        let mut raw = self.content.to_rgba().into_raw();
	    unsafe { 
	        let img = QImage::from_uchar2_int_format(MutPtr::from_raw(raw.as_mut_ptr()), w as i32, h as i32, Format::FormatRGBA8888);
            let pixmap = QPixmap::from_image_1a(img.as_ref());
            self.pixmap = match self.scale {
                types::ImageScalePolicy::FitCenter => pixmap.scaled_2_int_aspect_ratio_mode(iw as i32, ih as i32, AspectRatioMode::KeepAspectRatio),
                types::ImageScalePolicy::CropCenter => pixmap.copy_1a(&QRect::from_4_int((w as i32 - iw as i32) / 2, (h as i32 - ih as i32) / 2, iw as i32, ih as i32)),
            };
            self.base.widget.set_pixmap(self.pixmap.as_ref());
	    }
    }
}

impl HasLayoutInner for QtImage {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtImage {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = true;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {}

    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_IMAGE;
        fill_from_markup_base!(self, base, markup, registry, Image, [MEMBER_TYPE_IMAGE]);
    }
}

impl HasNativeIdInner for QtImage {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl HasVisibilityInner for QtImage {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtImage {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        unsafe { self.base.widget.set_fixed_size_2a(width as i32, height as i32); }
        true
    }
}
impl MemberInner for QtImage {}

impl Drawable for QtImage {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                use image::GenericImageView;
                
                let size = self.content.dimensions();
                let (w, h) = unsafe {
                    let margins = self.base.widget.contents_margins();
                    let w = match control.layout.width {
                        layout::Size::MatchParent => parent_width as i32,
                        layout::Size::Exact(w) => w as i32,
                        layout::Size::WrapContent => size.0 as i32 + margins.left() + margins.right(),
                    };
                    let h = match control.layout.height {
                        layout::Size::MatchParent => parent_height as i32,
                        layout::Size::Exact(h) => h as i32,
                        layout::Size::WrapContent => size.1 as i32 + margins.top() + margins.bottom(),
                    };
                    (w, h)
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate();
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Image::with_content("").into_control()
}*/

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match unsafe { event.type_() } {
        QEventType::Resize => {
            if let Some(this) = cast_qobject_to_uimember_mut::<Image>(object) {
                let size = unsafe { 
                    let size = MutPtr::from_raw(event).static_downcast_mut::<QResizeEvent>();
                    (
                    	utils::coord_to_size(size.size().width()), 
                    	utils::coord_to_size(size.size().height())
                    )
                };
                this.as_inner_mut().base_mut().measured = size;
                {
                	let (_, c, this) = this.as_base_parts_mut();
                	this.update_image(c);
                }
	            this.call_on_size(size.0, size.1);
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Image>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().base.widget, CppBox::new(MutPtr::null()).unwrap());
                }
            }
        }
        _ => {}
    }
    false
}

default_impls_as!(Image);
