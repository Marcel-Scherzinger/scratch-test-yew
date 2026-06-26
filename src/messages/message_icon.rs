use crate::MaterialIcon;
use sreport::messages::MessageKind;
use yew::prelude::*;

#[derive(Properties, derive_more::PartialEq)]
pub struct MessageIconProps {
    pub kind: MessageKind,
}
#[function_component(MessageIconComponent)]
pub fn message_kind(MessageIconProps { kind }: &MessageIconProps) -> Html {
    let icon = match kind {
        MessageKind::Info => "info",
        MessageKind::Warning => "warning",
        MessageKind::Error => "chat_error",
    };
    html!(
        <div class="message-icon">
            <MaterialIcon {icon}/>
        </div>
    )
}
