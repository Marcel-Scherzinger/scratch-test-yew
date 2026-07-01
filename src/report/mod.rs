mod category;

pub use category::CategoryComponent;

use sreport::{prelude::Simulation, report::Report};
use yew::prelude::*;

use crate::messages::MessageHubComponent;

#[derive(Properties, derive_more::PartialEq)]
pub struct ReportProps {
    pub report: Report,
    #[prop_or_default]
    pub class: Classes,
}
#[function_component(ReportComponent)]
pub fn report_component(ReportProps { report, class }: &ReportProps) -> Html {
    let mut report = report.clone();
    report.add_extra_messages();
    let messages = report.messages().clone();
    let simulation = report.simulation();

    html!(<div class={classes!("report-container", class)}>
        <MessageHubComponent<Report> open={true} {messages} classes={"report-messages"}/>

        if let Some(simulation) = simulation {
            <div class="simulation-container">
                <h2>{"Ergebnisse des Ausführens:"}</h2>
                <MessageHubComponent<Simulation>
                    open={true}
                    messages={simulation.messages().clone()}
                    classes={"simulation-messages"}/>
                {for simulation.categories().iter().cloned().map(
                    |category| html!(<CategoryComponent {category}/>)
                )}
            </div>
        }
    </div>)
}
