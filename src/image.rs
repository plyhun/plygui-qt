use crate::common::{self, *};
use crate::external::image;

use qt_core::qt::{AlignmentFlag, AspectRatioMode};
use qt_core::rect::{Rect as QRect};
use qt_gui::image::{Format, Image as QImage};
use qt_gui::pixmap::Pixmap as QPixmap;
use qt_widgets::label::Label as QLabel;

pub type Image = Member<Control<QtImage>>;

#[repr(C)]
pub struct QtImage {
    base: QtControlBase<Image, QLabel>,

    scale: types::ImageScalePolicy,
    pixmap: CppBox<QPixmap>,
    content: image::DynamicImage,
}

impl ImageInner for QtImage {
    fn with_content(content: image::DynamicImage) -> Box<controls::Image> {
        let mut i = Box::new(Member::with_inner(
            Control::with_inner(
                QtImage {
                    base: QtControlBase::with_params(QLabel::new(()), event_handler),
                    scale: types::ImageScalePolicy::FitCenter,
                    pixmap: unsafe { CppBox::new(ptr::null_mut()) },
                    content: content,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        unsafe {
            use qt_core::cpp_utils::StaticCast;
            let ptr = i.as_ref() as *const _ as u64;
            let qo: &mut QObject = i.as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        {
            let (_, c, i) = i.as_parts_mut();
            i.update_image(c);
        }
        i.as_inner_mut().as_inner_mut().base.widget.set_alignment(Flags::from_enum(AlignmentFlag::Center));
        i
    }
    fn set_scale(&mut self, _: &mut MemberBase, _: &mut ControlBase, policy: types::ImageScalePolicy) {
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

        let (iw, ih) = control.measured;
        let (w, h) = self.content.dimensions();
        let raw = self.content.to_rgba().into_raw();
        let img = unsafe { QImage::new_unsafe((raw.as_ptr(), w as i32, h as i32, Format::FormatRGBA8888)) };
        self.pixmap = match self.scale {
            types::ImageScalePolicy::FitCenter => {
                QPixmap::from_image(img.as_ref()).scaled((iw as i32, ih as i32, AspectRatioMode::KeepAspectRatio))
            },
            types::ImageScalePolicy::CropCenter => {
                QPixmap::from_image(img.as_ref()).copy(&QRect::new(((w as i32 - iw as i32)/2, (h as i32 - ih as i32)/2, iw as i32, ih as i32)))
            }
        };
        self.base.widget.set_pixmap(self.pixmap.as_ref());
    }
}

impl HasLayoutInner for QtImage {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtImage {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {}

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fill_from_markup_base!(self, base, markup, registry, Image, [MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl HasNativeIdInner for QtImage {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
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
        self.base.widget.set_fixed_size((width as i32, height as i32));
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
                let margins = self.base.widget.contents_margins();
                let size = if self.pixmap.is_null() {
                    QSize::new((0, 0))
                } else {
                    self.pixmap.size()
                };
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => size.width() as i32 + margins.left() + margins.right(),
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => size.height() as i32 + margins.top() + margins.bottom(),
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
    match event.type_() {
        QEventType::Resize => {
            let ptr = unsafe { object.property(PROPERTY.as_ptr() as *const i8).to_u_long_long() };
            if ptr != 0 {
                let sc: &mut Image = unsafe { mem::transmute(ptr) };
                let sc2: &mut Image = unsafe { mem::transmute(ptr) };
                sc.as_inner_mut().as_inner_mut().update_image(sc2.as_inner_mut().base_mut());
                if sc.as_inner().as_inner().base.dirty {
                    sc.as_inner_mut().as_inner_mut().base.dirty = false;
                    let (width, height) = sc.as_inner().base().measured;
                    sc.call_on_size(width, height);
                }
            }
        },
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Image>(object) {
                unsafe { ptr::write(&mut ll.as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut())); }
            }
        },
        _ => {}
    }
    false
}

default_impls_as!(Image);
