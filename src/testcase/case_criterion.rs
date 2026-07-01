use sreport::prelude::TestCriterion;
use yew::prelude::*;

use crate::{MaterialIcon, utils::Literal};

#[derive(Properties, derive_more::PartialEq)]
pub struct TestCaseCriterionProps {
    pub criterion: TestCriterion,
    pub last_output: Option<AttrValue>,
}

#[function_component(TestCaseCriterionComponent)]
pub fn message_kind(
    TestCaseCriterionProps {
        criterion,
        last_output,
    }: &TestCaseCriterionProps,
) -> Html {
    let is_happy = criterion.is_successful();

    let output = if let Some(text) = last_output {
        html!(<Literal {text}/>)
    } else {
        html!()
    };

    let b = match criterion {
        TestCriterion::LastOutputExact {
            expected,
            output_matches: _,
        } => html!(<>
        <i>
            if is_happy {
                {"Ihre Ausgabe passt genau zu der erwarteten Ausgabe!"}
            } else {
                {"Ihre Ausgabe unterscheidet sich von der erwarteten Ausgabe!"}
            }
        </i>
            <table>
                <tr>
                    <td>{"Ihre Ausgabe: "}</td>
                    <td>{output}</td>
                </tr>
                <tr>
                    <td>{"Erwartete Ausgabe: "}</td>
                    <td><Literal text={expected.to_string()}/></td>
                </tr>
            </table>
        </>),
        TestCriterion::LastOutputContains {
            sample_expected,
            expected,
            received_contains: _,
        } => html!(<>
        <i>
            if is_happy {
                {"Ihre Ausgabe enthält den erwarteten Textteil!"}
            } else {
                {"In Ihrer Ausgabe fehlt der erwartete Textteil!"}
            }
        </i>
            <table>
                <tr>
                    <td>{"Ihre Ausgabe: "}</td>
                    <td>{output}</td>
                </tr>
                <tr>
                    <td>{"Erwarteter Textteil: "}</td>
                    <td><Literal text={expected.to_string()}/></td>
                </tr>
                <tr>
                    <td>{"Beipielausgabe: "}</td>
                    <td><Literal text={sample_expected.to_string()}/></td>
                </tr>
            </table>
        </>),
        TestCriterion::LastOutputInterpreted {
            sample_expected,
            iexpected,
            ireceived,
            interpretations_match: _,
        } => html!(<>
        <i>
            if is_happy {
                {"Ihr Programm hat die richtige Entscheidung getroffen!"}
            } else {
                {"Ihr Programm hat sich anscheinend für die falsche Ausgabe entschieden! (Das kann ein Fehler in Ihrem Programm sein. Vielleicht enthält ihre Ausgabe aber auch nur einen ungünstigen Begriff, der mich dazu verleitet, Ihre Ausgabe falsch zu interpretieren.)"}
            }
        </i>
            <table>
                <tr>
                    <td>{"Ihre Ausgabe: "}</td>
                    <td>{output}</td>
                </tr>
                <tr>
                    <td>{"Erkannte Entscheidung: "}</td>
                    <td>if let Some(text) = ireceived.as_ref() {
                        <Literal text={text.to_string()}/>
                    }</td>
                </tr>
                <tr>
                    <td>{"Erwartete Entscheidung: "}</td>
                    <td>
                        <Literal text={iexpected.to_string()}/>
                    </td>
                </tr>
                <tr>
                    <td>{"Beipielausgabe: "}</td>
                    <td><Literal text={sample_expected.to_string()}/></td>
                </tr>
            </table>
        </>),
        TestCriterion::LastOutputContainsNumber {
            expected,
            sample_expected,
            found_numbers,
            received_contains: _,
        } => html!(<>
        <i>
            if is_happy {
                {"Ihre Ausgabe enthält die erwartete Zahl!"}
            } else {
                {"In Ihrer Ausgabe fehlt die erwartete Zahl!"}
            }
        </i>
            <table>
                <tr>
                    <td>{"Ihre Ausgabe: "}</td>
                    <td>{output}</td>
                </tr>
                <tr>
                    <td>{"Erwartete Zahl: "}</td>
                    <td><Literal text={expected.to_string()}/></td>
                </tr>
                <tr>
                    <td>{"In ihrer Ausgabe gefunden: "}</td>
                    <td><ul class="found-numbers-list">
                        {for found_numbers.iter().map(
                            |n| html!(<li><span>{n.to_string()}</span></li>)
                        )}
                    </ul></td>
                </tr>
                <tr>
                    <td>{"Beipielausgabe: "}</td>
                    <td><Literal text={sample_expected.to_string()}/></td>
                </tr>
            </table>
        </>),
        TestCriterion::LastOutputContainsOnlyThisNumber {
            expected,
            sample_expected,
            found_numbers,
            received_contains_only_this: _,
        } => html!(<>
        <i>
            if is_happy {
                {"Ihre Ausgabe enthält nur die erwartete Zahl!"}
            } else {
                {"In Ihrer Ausgabe fehlt die erwartete Zahl oder es kommen weitere vor!"}
            }
        </i>
            <table>
                <tr>
                    <td>{"Ihre Ausgabe: "}</td>
                    <td>{output}</td>
                </tr>
                <tr>
                    <td>{"Erwartete Zahl: "}</td>
                    <td><Literal text={expected.to_string()}/></td>
                </tr>
                <tr>
                    <td>{"In ihrer Ausgabe gefunden: "}</td>
                    <td><ul class="found-numbers-list">
                        {for found_numbers.iter().map(
                            |n| html!(<li><span>{n.to_string()}</span></li>)
                        )}
                    </ul></td>
                </tr>
                <tr>
                    <td>{"Beipielausgabe: "}</td>
                    <td><Literal text={sample_expected.to_string()}/></td>
                </tr>
            </table>
        </>),
    };

    let icon = if is_happy { "check" } else { "chat_error" };

    html!(
        <div class="testcase-criterion case-part-box" data-criterion-ok={is_happy.to_string()}>
            <summary><span class="title">{"Mein Kriterium:"}</span><MaterialIcon {icon}/></summary>
            {b}
        </div>
    )
}
