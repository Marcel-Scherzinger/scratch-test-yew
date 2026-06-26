use sreport::{messages::MessageKind, prelude::Message};
use yew::prelude::*;

use crate::messages::MessageIconComponent;

#[derive(Properties, derive_more::PartialEq)]
pub struct MessageProps<Level> {
    pub msg: Message<Level>,
}

#[function_component(MessageComponent)]
pub fn message<Level>(MessageProps { msg }: &MessageProps<Level>) -> Html {
    let kind = msg.kind();
    let text = msg.msg();

    let severity_class = match kind {
        MessageKind::Error => "message-error",
        MessageKind::Warning => "message-warning",
        MessageKind::Info => "message-info",
    };
    html!(
        <li class={classes!("message-container", severity_class)}>
            <MessageIconComponent kind={*kind}/>
            <div class="message-content">
                {text}
            </div>
        </li>
    )
}
