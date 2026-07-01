use sreport::{
    messages::MessageAdder,
    prelude::{Message, TestCase},
};
use yew::prelude::*;

use crate::testcase::TestCaseComponent;

const MSG1: Message<TestCase> = Message::cinfo(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);

const MSG2: Message<TestCase> = Message::cwarning(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);

const MSG3: Message<TestCase> = Message::cerror(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);

#[cfg(not(debug_assertions))]
#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html!()
}

#[cfg(debug_assertions)]
#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    let mut case = TestCase::create();
    case.notify(MSG1);
    case.notify(MSG2);
    case.notify(MSG3);
    case.inputs(vec![
        "10 Das ist aber wirklich eine Menge Text, wieso bekomme ich solch lange Eingaben?".into(),
        20.into(),
    ]);
    case.randoms(vec![
        svalue::SNumber::Int(10),
        svalue::SNumber::Int(42),
        svalue::SNumber::Int(42),
        svalue::SNumber::Int(42),
        svalue::SNumber::Int(42),
        svalue::SNumber::Int(42),
        svalue::SNumber::Int(42),
    ]);
    case.received_output(vec!["200".into()]);
    case.criterion(sreport::prelude::TestCriterion::LastOutputInterpreted {
        sample_expected: "Das Produkt ist größer als 50".into(),
        iexpected: "größer als 50".into(),
        ireceived: Some("größer als 50".into()),
        interpretations_match: Some(false),
    });
    case.received_variables(vec![("Var1".into(), "Test".into())].into_iter().collect());
    case.received_lists(
        vec![
            ("Liste".into(), vec!["A".into(), "B".into()]),
            ("Liste2".into(), vec!["C".into(), "D".into(), "E".into()]),
        ]
        .into_iter()
        .collect(),
    );

    let (case, _, _) = case.derived_status().build();
    html!(
        <main>
            <div>
                <h1 style={"margin: auto; text-align: center;"}>
                    {"Willkommen zu Bast3St!"}
                </h1>
                <TestCaseComponent {case} />
            </div>
        </main>
    )
}
