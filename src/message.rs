use crate::common::{self, *};

use qt_widgets::q_message_box::{ButtonRole, Icon};
use qt_widgets::QMessageBox;
use qt_core::WindowModality;

use std::borrow::Cow;

pub type Message = Member<QtMessage>;

#[repr(C)]
pub struct QtMessage {
    message: CppBox<QMessageBox>,
    filter: CppBox<CustomEventFilter>,
    actions: Vec<(String, callbacks::Action)>,
}

impl HasLabelInner for QtMessage {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = self.message.as_ref().text().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data().as_raw_ptr() as *const u8, name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.message.set_text(&QString::from_std_str(&label)); }
    }
}

impl MessageInner for QtMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<Member<Self>> {
        let mut message = Box::new(Member::with_inner(
            QtMessage {
                message: unsafe { QMessageBox::new() },
                filter: CustomEventFilter::new(event_handler),
                actions: actions,
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = message.as_ref() as *const _ as u64;
            (message.as_inner_mut().message.static_upcast_mut::<QObject>()).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        unsafe {
            let message = message.as_inner_mut();
            let mut qmessage = message.message.as_mut_ptr();

            match content {
                types::TextContent::Plain(ref text) => {
                    message.message.set_text(&QString::from_std_str(text.as_str()));
                }
                types::TextContent::LabelDescription(label, description) => {
                    message.message.set_text(&QString::from_std_str(label.as_str()));
                    message.message.set_informative_text(&QString::from_std_str(description.as_str()));
                }
            }

            message.message.set_icon(severity_to_message_icon(severity));

            if let Some(parent) = parent {
                message.message.set_parent(common::cast_member_to_qwidget(parent).window());
                message.message.set_window_modality(WindowModality::WindowModal);
            }
            message.actions.iter().for_each(|a| {
                (&mut *qmessage).add_button_q_string_button_role(&QString::from_std_str(a.0.as_str()), ButtonRole::ActionRole);
            });

            let filter = message.filter.static_upcast_mut::<QObject>();
            let mut qobject = message.message.static_upcast_mut::<QObject>();
            qobject.install_event_filter(filter);
        }
        message
    }
    fn severity(&self) -> types::MessageSeverity {
        message_icon_to_severity(unsafe { self.message.icon() })
    }
    fn start(mut self) -> Result<String, ()> {
        let mut ptr = unsafe { self.message.static_upcast_mut::<QObject>() };
        self.actions
            .get_mut(unsafe { self.message.exec() as usize })
            .map(|a| {
                let message2 = { common::cast_qobject_to_uimember_mut::<Message>(&mut *ptr).unwrap() };
                (a.1.as_mut())(message2);
                a.0.clone()
            })
            .ok_or(())
    }
}
impl HasNativeIdInner for QtMessage {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.message.static_upcast::<QObject>().as_raw_ptr() as *mut QObject)
    }
}
impl MemberInner for QtMessage {}

impl Drop for QtMessage {
    fn drop(&mut self) {
        self.filter.clear();
    }
}

fn message_icon_to_severity(icon: Icon) -> types::MessageSeverity {
    match icon {
        Icon::Information => types::MessageSeverity::Info,
        Icon::Warning => types::MessageSeverity::Warning,
        Icon::Critical => types::MessageSeverity::Alert,
        _ => unreachable!(),
    }
}
fn severity_to_message_icon(severity: types::MessageSeverity) -> Icon {
    match severity {
        types::MessageSeverity::Info => Icon::Information,
        types::MessageSeverity::Warning => Icon::Warning,
        types::MessageSeverity::Alert => Icon::Critical,
    }
}

fn event_handler(_: &mut QObject, _: &mut QEvent) -> bool {
    false
}

default_impls_as!(Message);
