use itertools::Either;
use yew::prelude::*;

#[function_component(InfoSection)]
pub fn info_section() -> Html {
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
            <>
                <p class="welcome-info">{"Ich bin dazu da, um Ihnen bei Ihren Scratch-Abgaben zu helfen. Sie können mir Ihre Dateien geben und ich teste Sie für die jewiligen Aufgaben mit verschiedenen Eingaben. Wenn alles passt, wissen Sie auf diese Weise sofort, dass Ihre Lösung ausreichend ist. Ab und zu kann ich Ihnen auch Tipps bei Fehlern geben, sofern ich erkenne, worin das Problem besteht."}</p>
                <p class="welcome-info">{"Noch befinde ich mich in der Versuchsphase. Wenn Ihnen also etwas auffällt, das ich besser machen sollte, oder wenn Sie mit einem Hinweis nicht zurechtkommen, zögern Sie nicht, über das Forum nachzufragen."}</p>

                <section class="welcome-info">
                <p>{"Mein Name ist sehr vielschichtig und funktioniert auf mehreren Ebenen:"}</p>
                <table class="name-info">
                    {for name_comp}
                </table>
                </section>
            </>
    )
}
