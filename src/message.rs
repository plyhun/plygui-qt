use super::common::*;
use super::*;

use qt_widgets::message_box::{ButtonRole, Icon, MessageBox as QMessageBox};

use std::borrow::Cow;

pub type Message = Member<QtMessage>;

#[repr(C)]
pub struct QtMessage {
    message: CppBox<QMessageBox>,
    filter: CppBox<CustomEventFilter>,
    actions: Vec<(String, callbacks::Action)>,
}

impl HasLabelInner for QtMessage {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let name = (&*self.message.as_ref()).text().to_utf8();
        unsafe {
            let bytes = std::slice::from_raw_parts(name.const_data() as *const u8, name.count(()) as usize);
            Cow::Owned(std::str::from_utf8_unchecked(bytes).to_owned())
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        self.message.set_text(&QString::from_std_str(label));
    }
}

impl MessageInner for QtMessage {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<Member<Self>> {
        let mut message = Box::new(Member::with_inner(
            QtMessage {
                message: QMessageBox::new(()),
                filter: CustomEventFilter::new(event_handler),
                actions: actions,
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        unsafe {
            let ptr = message.as_ref() as *const _ as u64;
            (message.as_inner_mut().message.as_mut().static_cast_mut() as &mut QObject).set_property(common::PROPERTY.as_ptr() as *const i8, &QVariant::new0(ptr));
        }
        unsafe {
            let message = message.as_inner_mut();
            let qmessage = message.message.as_mut_ptr();
            
            match content {
                types::TextContent::Plain(ref text) => {
                    message.message.set_text(&QString::from_std_str(text.as_str()));
                },
                types::TextContent::LabelDescription(label, description) => {
                    message.message.set_text(&QString::from_std_str(label.as_str()));
                    message.message.set_informative_text(&QString::from_std_str(description.as_str()));
                }
            }
            
            message.message.set_icon(severity_to_message_icon(severity));
            
            if let Some(parent) = parent {
                message.message.set_parent(common::cast_member_to_qwidget(parent).window());
            }
            message.actions.iter().for_each(|a| {
                (&mut *qmessage).add_button((&QString::from_std_str(a.0.as_str()), ButtonRole::ActionRole));
            });
            
            let filter: *mut QObject = message.filter.static_cast_mut() as *mut QObject;
            let qobject: &mut QObject = message.message.as_mut().static_cast_mut();
            qobject.install_event_filter(filter);
        }
        message
    }
    fn severity(&self) -> types::MessageSeverity {
        message_icon_to_severity(self.message.icon())
    }
    fn start(mut self) -> Result<String, ()> {
        let ptr = self.message.static_cast_mut() as *mut QObject;
        self.actions.get_mut(self.message.exec() as usize).map(|a| {
            let message2 = {
                unsafe { common::cast_qobject_to_uimember_mut::<Message>(&mut *ptr).unwrap() }
            };
            (a.1.as_mut())(message2);
            a.0.clone()
        }).ok_or(())
    }
}
impl HasNativeIdInner for QtMessage {
    type Id = common::QtId;

    unsafe fn native_id(&self) -> Self::Id {
        QtId::from(self.message.static_cast() as *const QObject as *mut QObject)
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

fn event_handler(_: &mut QObject, event: &mut QEvent) -> bool {
    match event.type_() {
        QEventType::Leave => {
            dbg!("close");
        }
        _ => {}
    }
    false
}

impl_all_defaults!(Message);
