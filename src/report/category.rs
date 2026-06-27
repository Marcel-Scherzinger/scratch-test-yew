use sreport::{
    prelude::{Category, Simulation},
    report::Report,
};
use yew::prelude::*;

use crate::{messages::MessageHubComponent, testcase::TestCaseComponent, utils::sinplu};

#[derive(Properties, derive_more::PartialEq)]
pub struct CategoryProps {
    pub category: Category,
}
#[function_component(CategoryComponent)]
pub fn category_component(CategoryProps { category }: &CategoryProps) -> Html {
    let description = category.description();
    let status = category.status();
    let messages = category.messages().clone();
    let cases = category.cases();

    html!(<details class="category-container">
        <summary class="category-status">
            {sinplu(*status.failure(), "Fehlschlag", "Fehlschläge")}
            {", "}
            {sinplu(*status.success_but_warnings(), "Teilerfolg", "Teilerfolge")}
            {", "}
            {sinplu(*status.complete_success(), "Erfolg", "Erfolge")}
            {":"}
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
    </details>)
}
