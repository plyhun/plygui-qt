use crate::common::{self, *};

use qt_core::connection::Signal;
use qt_core::rect::Rect as QRect;
use qt_core::slots::SlotNoArgs;
use qt_gui::font_metrics::FontMetrics as QFontMetrics;
use qt_widgets::push_button::PushButton as QPushButton;

use std::borrow::Cow;
use std::cmp::max;

pub type Button = Member<Control<QtButton>>;

#[repr(C)]
pub struct QtButton {
    base: common::QtControlBase<Button, QPushButton>,

    h_left_clicked: (bool, SlotNoArgs<'static>),
}

impl HasLabelInner for QtButton {
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

impl ClickableInner for QtButton {
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.h_left_clicked.0 = cb.is_some();
        if cb.is_some() {
            let mut cb = cb.unwrap();
            let ptr = self.base.widget.as_mut().static_cast_mut() as *mut QObject;
            self.h_left_clicked.1.set(move || unsafe {
                let button = cast_qobject_to_uimember_mut::<Button>(&mut *ptr).unwrap();
                (cb.as_mut())(button);
            });
        } else {
            self.h_left_clicked.1.clear();
        }
    }
    fn click(&mut self) {
        self.base.widget.click();
    }
}

impl ButtonInner for QtButton {
    fn with_label(label: &str) -> Box<Button> {
        let mut btn = Box::new(Member::with_inner(
            Control::with_inner(
                QtButton {
                    base: common::QtControlBase::with_params(QPushButton::new(&QString::from_std_str(label)), event_handler),
                    h_left_clicked: (false, SlotNoArgs::new(move || {})),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = btn.as_ref() as *const _ as u64;
            btn.as_inner().as_inner().base.widget.signals().released().connect(&btn.as_inner().as_inner().h_left_clicked.1);
            let qo: &mut QObject = btn.as_inner_mut().as_inner_mut().base.widget.static_cast_mut();
            qo.set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        btn
    }
}

impl HasLayoutInner for QtButton {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for QtButton {
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
        control.coords = Some((x, y));
        self.measure(member, control, pw, ph);
        self.base.dirty = false;
        self.draw(member, control);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {}

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;

        fill_from_markup_base!(self, markup, registry, Button, [MEMBER_ID_BUTTON, MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, markup);
        fill_from_markup_callbacks!(self, markup, registry, ["on_click" => FnMut(&mut controls::Button)]);
    }
}
impl HasNativeIdInner for QtButton {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.base.widget.static_cast() as *const QObject as *mut QObject)
    }
}
impl HasVisibilityInner for QtButton {
    fn on_visibility_set(&mut self, _: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.set_visibility(value);
        self.base.invalidate()
    }
}
impl HasSizeInner for QtButton {
    fn on_size_set(&mut self, _: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        self.base.widget.set_fixed_size((width as i32, height as i32));
        true
    }
}
impl MemberInner for QtButton {}

impl Drawable for QtButton {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(member, control);
    }
    fn measure(&mut self, _: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
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
        self.base.dirty = control.measured != old_size;
        (control.measured.0, control.measured.1, self.base.dirty)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate();
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Button::with_label("").into_control()
}

fn event_handler(object: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Resize => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Button>(object) {
                use plygui_api::controls::HasSize;

                if ll.as_inner().as_inner().base.dirty {
                    ll.as_inner_mut().as_inner_mut().base.dirty = false;
                    let (width, height) = ll.size();
                    ll.call_on_size(width, height);
                }
            }
        }
        QEventType::Destroy => {
            if let Some(ll) = cast_qobject_to_uimember_mut::<Button>(object) {
                unsafe {
                    ptr::write(&mut ll.as_inner_mut().as_inner_mut().base.widget, CppBox::new(ptr::null_mut()));
                }
            }
        }
        _ => {}
    }
    false
}
default_impls_as!(Button);
