use crate::common::{self, *};

use qt_widgets::q_message_box::{ButtonRole, Icon};
use qt_widgets::QMessageBox;
use qt_core::WindowModality;

use std::borrow::Cow;

pub type Message = AMember<AMessage<QtMessage>>;

#[repr(C)]
pub struct QtMessage {
    message: QBox<QMessageBox>,
    filter: QBox<CustomEventFilter>,
    actions: Vec<(String, callbacks::Action)>,
}

impl HasLabelInner for QtMessage {
    fn label(&self, _: &MemberBase) -> Cow<str> {
        unsafe {
            let name = self.message.as_ref().unwrap().text().to_utf8();
            let bytes = std::slice::from_raw_parts(name.const_data(), name.count() as usize);
            Cow::Owned(std::str::from_utf8_unchecked(mem::transmute(bytes)).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        unsafe { self.message.set_text(&QString::from_std_str(&label)); }
    }
}

impl MessageInner for QtMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<dyn controls::Message> {
        let mut message = Box::new(AMember::with_inner(
            AMessage::with_inner(
                QtMessage {
                    message: unsafe { QMessageBox::new() },
                    filter: CustomEventFilter::new(event_handler),
                    actions: actions,
                }
            ),
        ));
        unsafe {
            let ptr = message.as_ref() as *const _ as u64;
            (message.inner_mut().inner_mut().message.static_upcast::<QObject>()).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::from_u64(ptr));
        }
        unsafe {
            let message = message.inner_mut().inner_mut();
            let qmessage = message.message.as_ptr();

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
                qmessage.add_button_q_string_button_role(&QString::from_std_str(a.0.as_str()), ButtonRole::ActionRole);
            });

            let filter = message.filter.static_upcast::<QObject>();
            let qobject = message.message.static_upcast::<QObject>();
            qobject.install_event_filter(filter);
        }
        message
    }
    fn severity(&self) -> types::MessageSeverity {
        message_icon_to_severity(unsafe { self.message.icon() })
    }
    fn start(mut self) -> Result<String, ()> {
        let ptr = unsafe { self.message.static_upcast::<QObject>() };
        self.actions
            .get_mut(unsafe { self.message.exec() as usize })
            .map(|a| {
                let message2 = { common::cast_qobject_to_uimember_mut::<Message>(&*ptr).unwrap() };
                (a.1.as_mut())(message2);
                a.0.clone()
            })
            .ok_or(())
    }
}
impl HasNativeIdInner for QtMessage {
    type Id = common::QtId;

    fn native_id(&self) -> Self::Id {
        QtId::from(unsafe { self.message.static_upcast::<QObject>().as_raw_ptr() } as *mut QObject)
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
