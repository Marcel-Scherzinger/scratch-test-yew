use sreport::messages::{MessageKind, Messages};
use yew::prelude::*;

use crate::messages::MessageComponent;
use crate::utils::sinplu;

#[derive(Properties, derive_more::PartialEq)]
pub struct MessagesProps<Level> {
    pub messages: Messages<Level>,
    #[prop_or(true)]
    pub open: bool,
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component(MessageHubComponent)]
pub fn message<Level>(
    MessagesProps {
        messages,
        open,
        classes,
    }: &MessagesProps<Level>,
) -> Html
where
    Level: Clone + 'static,
{
    let msgs = messages
        .iter()
        .rev()
        .map(|m| html!(<MessageComponent<Level> msg={m.clone()}/>));

    let (mut info, mut warn, mut err) = (0, 0, 0);
    for m in messages.iter() {
        match m.kind() {
            MessageKind::Info => info += 1,
            MessageKind::Warning => warn += 1,
            MessageKind::Error => err += 1,
        }
    }

    let highest_kind = if err > 0 {
        "error"
    } else if warn > 0 {
        "warning"
    } else if info > 0 {
        "info"
    } else {
        "none"
    };

    if highest_kind == "none" {
        return html!();
    }
    html!(
        <details class={classes!("messages-container", "case-part-box", classes.clone())}
            open={*open} data-highest-kind={highest_kind}>
            <summary>
                {sinplu(info + warn + err, "Nachricht", "Nachrichten")}
                {" ("}
                {sinplu(err, "Fehlermeldung", "Fehlermeldungen")}
                {", "}
                {sinplu(warn, "Warnung", "Warnungen")}
                {", "}
                {sinplu(info, "Hinweis", "Hinweise")}
                {"):"}
            </summary>
            <ul class="messages-list">
                {for msgs}
            </ul>
        </details>
    )
}
