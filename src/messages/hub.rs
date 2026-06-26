use sreport::messages::{MessageKind, Messages};
use yew::prelude::*;

use crate::messages::MessageComponent;

#[derive(Properties, derive_more::PartialEq)]
pub struct MessagesProps<Level> {
    pub messages: Messages<Level>,
    #[prop_or(true)]
    pub open: bool,
}

fn sinplu(num: usize, sin: &'static str, plu: &'static str) -> String {
    if num == 0 {
        format!("keine {plu}")
    } else if num == 1 {
        format!("1 {sin}")
    } else {
        format!("{num} {plu}")
    }
}

#[function_component(MessageHubComponent)]
pub fn message<Level>(MessagesProps { messages, open }: &MessagesProps<Level>) -> Html
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
        <details class="messages-container case-part-box" open={*open} data-highest-kind={highest_kind}>
            <summary>
                {sinplu(info + warn + err, "Nachricht", "Nachrichten")}
                {" ("}
                {sinplu(err, "Fehlermeldung", "Fehlermeldungen")}
                {", "}
                {sinplu(warn, "Warnung", "Warnungen")}
                {", "}
                {sinplu(info, "Hinweis", "Hinweise")}
                {")"}
            </summary>
            <ul class="messages-list">
                {for msgs}
            </ul>
        </details>
    )
}
