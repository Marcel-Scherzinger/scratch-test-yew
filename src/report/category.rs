use itertools::Itertools;
use sreport::prelude::Category;
use yew::prelude::*;

use crate::{
    messages::MessageHubComponent,
    testcase::TestCaseComponent,
    utils::{sinplu, sinplu_non_zero},
};

#[derive(Properties, derive_more::PartialEq)]
pub struct CategoryProps {
    pub category: Category,
}
#[function_component(CategoryComponent)]
pub fn category_component(CategoryProps { category }: &CategoryProps) -> Html {
    let title = category.title();
    let description = category.description();
    let status = category.status();
    let messages = category.messages().clone();
    let cases = category.cases();

    let total = status.success_but_warnings() + status.complete_success() + status.failure();
    let missing = total.saturating_sub(cases.len());

    let status_text = vec![
        sinplu_non_zero(*status.failure(), "Fehlschlag", "Fehlschläge"),
        sinplu_non_zero(*status.success_but_warnings(), "Teilerfolg", "Teilerfolge"),
        sinplu_non_zero(*status.complete_success(), "Erfolg", "Erfolge"),
    ]
    .into_iter()
    .flatten()
    .join(", ");
    let status_text = if title.is_some() {
        format!("({status_text})")
    } else {
        status_text
    };

    html!(<details class="category-container" open={true}>
        <summary>
            if let Some(title) = title {
                <span class="category-title">{title.to_string()}{" "}</span>
            }
            <span class="category-status">{status_text}</span>
        </summary>
        if let Some(description) = description {
            <p class="category-description">{description.to_string()}</p>
        }

        <MessageHubComponent<Category> open={true} {messages} classes={"category-messages"}/>

        <ul class="category-tests">
            {for cases.iter().map(|c| html!(
                <li><TestCaseComponent case={c.clone()}/></li>
            ))}
        </ul>
        if missing > 0 {
            <div class="missing-tests-info">
                {sinplu(missing, "Test wird", "Tests werden")}{" nicht angezeigt"}
            </div>
        }
    </details>)
}
