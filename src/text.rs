use super::common::*;
use super::*;

use qt_core::rect::Rect as QRect;
use qt_gui::font_metrics::FontMetrics as QFontMetrics;
use qt_widgets::label::Label as QLabel;

use std::borrow::Cow;
use std::cmp::max;

pub type Text = Member<Control<QtText>>;

#[repr(C)]
pub struct QtText {
    base: common::QtControlBase<Text, QLabel>,
}

impl HasLabelInner for QtText {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let name = self.base.widget.as_ref().text().to_utf8();
        unsafe {
            let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.base.widget.as_mut().set_text(&QString::from_std_str(label));
    }
}

impl TextInner for QtText {
    fn with_text(text: &str) -> Box<Text> {
        let mut btn = Box::new(Member::with_inner(
            Control::with_inner(
                QtText {
                    base: common::QtControlBase::with_params(QLabel::new(&QString::from_std_str(text)), event_handler),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = btn.as_ref() as *const _ as u64;
            let qo: &mut QObject = btn.as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        btn
    }
}

impl HasLayoutInner for QtText {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtText {
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
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {}

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_TEXT;

        fill_from_markup_base!(self, markup, registry, Text, [MEMBER_ID_TEXT, MEMBER_TYPE_TEXT]);
        fill_from_markup_label!(self, markup);
        fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut controls::Text)]);
    }
}

impl MemberInner for QtText {
    type Id = common::QtId;

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.base.set_visibility(base.visibility);
        self.base.invalidate()
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }
    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.as_ref() as *const _ as *mut QWidget)
    }
}

impl Drawable for QtText {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let font = self.base.widget.as_ref().font();

                let mut label_size = QRect::new((0, 0, 0, 0));
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.width() < 1 {
                            let fm = QFontMetrics::new(font);
                            label_size = fm.bounding_rect(&self.base.widget.as_ref().text());
                        }
                        label_size.width() + 16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.height() < 1 {
                            let fm = QFontMetrics::new(font);
                            label_size = fm.bounding_rect(&self.base.widget.as_ref().text());
                        }
                        label_size.height() + 16
                    }
                };
                (max(0, w) as u16, max(0, h) as u16)
            }
        };
        self.base.dirty = self.base.measured_size != old_size;
        (self.base.measured_size.0, self.base.measured_size.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Text::empty().into_control()
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Text>(object) {
                use plygui_api::controls::Member;

                if ll.as_inner().as_inner().base.dirty {
                    ll.as_inner_mut().as_inner_mut().base.dirty = false;
                    let (width, height) = ll.size();
                    ll.call_on_resize(width, height);
                }
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Text>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut()));
                }
            }
        }
        _ => {}
    }
    false
}
impl_all_defaults!(Text);
