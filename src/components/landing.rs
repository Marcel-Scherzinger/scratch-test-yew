use itertools::Either;
// use sreport::prelude::{Message, TestCase};
use yew::prelude::*;

/*
const MSG1: Message<TestCase> = Message::cinfo(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);

const MSG2: Message<TestCase> = Message::cwarning(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);

const MSG3: Message<TestCase> = Message::cerror(
    "Als ich Ihr Programm aufgerufen habe, hat Ihre Ausgabe einen der anderen abgeprüften Namen enthalten. Deshalb nehme ich an, dass Sie die Aufgabe nicht richtig lösen, sondern die Ergebnisse fest in Ihr Programm kodieren.",
);
*/

// #[cfg(not(debug_assertions))]
#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    use Either::Left as L;
    use Either::Right as R;
    let name_expl = [
        (
            vec![R("Bast3"), L("S"), R("t")],
            "Scratch ist mein Gebiet und das beginnt mit S.",
        ),
        (
            vec![L("B"), R("ast"), L("3S"), R("t")],
            "Die Dateiendung von Scratch ist sb3.",
        ),
        (
            vec![L("Baste"), R("S"), L("t")],
            "Bastet ist die ägyptische Katzengöttin und Scratch ist für seine Katze bekannt.",
        ),
        (
            vec![R("Bas"), L("teSt")],
            "Ich bin dazu da, um Ihre Abgaben zu testen.",
        ),
    ];

    let name_comp = name_expl.iter().map(|(v, desc)| {
        let h = v.iter().map(|n| match n {
            L(highlight) => html!(<span class="name-highlight">{highlight}</span>),
            R(unimport) => html!(<span class="name-unimportant">{unimport}</span>),
        });
        html!(<tr><td>{for h}{": "}</td><td>{desc}</td></tr>)
    });

    html!(
        <main>
            <div>
                <h1 style={"margin: auto; text-align: center;"}>
                    {"Willkommen zu Bast3St!"}
                </h1>
                <p class="welcome-info">{"Ich bin dazu da, um Ihnen bei Ihren Scratch-Abgaben zu helfen. Sie können mir Ihre Dateien geben und ich teste Sie für die jewiligen Aufgaben mit verschiedenen Eingaben. Wenn alles passt, wissen Sie auf diese Weise sofort, dass Ihre Lösung ausreichend ist. Ab und zu kann ich Ihnen auch Tipps bei Fehlern geben, sofern ich erkenne, worin das Problem besteht."}</p>
                <p class="welcome-info">{"Noch befinde ich mich in der Versuchsphase. Wenn Ihnen also etwas auffällt, das ich besser machen sollte, oder wenn Sie mit einem Hinweis nicht zurechtkommen, zögern Sie nicht, über das Forum nachzufragen."}</p>

                <section class="welcome-info">
                <p>{"Mein Name ist sehr vielschichtig und funktioniert auf mehreren Ebenen:"}</p>
                <table class="name-info">
                    {for name_comp}
                </table>
                </section>


            </div>
        </main>
    )
}

/*
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
*/
