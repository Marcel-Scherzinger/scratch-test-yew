use crate::components::MessageHubComponent;
use crate::testcase::{TestCaseCriterionComponent, TestCaseStatusIconComponent};
use crate::utils::Literal;
use sreport::prelude::{TestCase, TestCaseStatus};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestCaseProps {
    pub case: TestCase,
}

fn to_list_items<D: ToString>(iter: impl Iterator<Item = D>) -> impl Iterator<Item = Html> {
    iter.map(|i| html!(<li><Literal text={i.to_string()}/></li>))
}

#[derive(Properties, PartialEq)]
pub struct OutputProps {
    pub output: TestCase,
}

#[function_component(TestCaseComponent)]
pub fn test_case(TestCaseProps { case }: &TestCaseProps) -> Html {
    let messages = case.messages().clone();
    let status = case.status().clone();
    let inputs = case.inputs().clone();
    let randoms = case.randoms().clone();
    // let error_code = case.error_code().clone();
    let criterion = case.criterion().clone();
    let received = case.received().clone();

    let lists = received.data().lists();
    let variables = received.data().variables();

    let status_title = match status {
        TestCaseStatus::CompleteSucess => "Dieser Test war erfolgreich",
        TestCaseStatus::Failure => "Dieser Test schlug fehl",
        TestCaseStatus::SuccessButWarnings => "Es liegen Warnungen vor",
    };

    let last_output = received
        .output()
        .last()
        .map(|s| AttrValue::from(s.to_string()));

    let inputs_comp = to_list_items(inputs.iter());
    let outputs_comp = to_list_items(received.output().iter());
    let variables_comp = variables.iter().map(|(key, value)| {
        html!(
            <tr>
                <td>{key.to_string()}</td>
                <td><Literal text={value.to_string()}/></td>
            </tr>
        )
    });
    let lists_comp = lists.iter().map(|(key, value)| {
        let list = to_list_items(value.iter());
        html!(
            <tr>
                <td>{key.to_string()}</td>
                <td><ol>{for list}</ol></td>
            </tr>
        )
    });
    let randoms_div = if let Some(randoms) = randoms {
        let randoms_comp = to_list_items(randoms.iter());
        html!(
            <details class="testcase-randoms case-part-box">
                <summary>{"Zufallszahlen:"}</summary>
                <ol>
                    {for randoms_comp}
                </ol>
            </details>
        )
    } else {
        html!()
    };

    html!(<section class="testcase-container" data-status={status.to_string()}>
        <h3 class="testcase-status">
            <TestCaseStatusIconComponent {status}/>
            <span class="testcase-status-title"> {status_title} </span>
        </h3>

        <MessageHubComponent<TestCase> open={true} {messages} classes={"testcase-messages"}/>


        if let Some(criterion) = criterion {
            <TestCaseCriterionComponent {criterion} {last_output}/>
        }

        {randoms_div}

        <div class="testcase-inputs case-part-box">
            <summary>{"Meine Eingaben:"}</summary>
            <ol>
                {for inputs_comp}
            </ol>
        </div>

        <div class="testcase-outputs case-part-box">
            <summary>{"Ihre Ausgaben:"}</summary>
            <ol>
                {for outputs_comp}
            </ol>
        </div>

        <div class="testcase-variables case-part-box">
            <summary>{"Ihre Variablen:"}</summary>
            <table>
                {for variables_comp}
            </table>
        </div>

        <div class="testcase-lists case-part-box">
            <summary>{"Ihre Listen:"}</summary>
            <table>
                {for lists_comp}
            </table>
        </div>
    </section>)
}
