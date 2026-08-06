use super::info_section::InfoSection;
use sreport::report::Report;
use yew::prelude::*;

use crate::{
    UserIdentifier,
    api::SharableError,
    components::{FileDetails, FileUpload},
    report::ReportComponent,
};

#[derive(Properties, PartialEq)]
pub struct ExerciseProps {
    pub exercise: String,
}

#[function_component(ExercisePage)]
pub fn exercise(ExerciseProps { exercise }: &ExerciseProps) -> Html {
    let user_id: UserIdentifier = if let Some(ctx) = use_context() {
        ctx
    } else {
        log::error!("user identifier should always be there at this point");
        UserIdentifier("".into())
    };

    let files_handle: UseStateHandle<Option<FileDetails>> = use_state(move || None);
    let current_report = use_state_eq(move || None);
    let file_selected = Callback::from({
        let files_handle = files_handle.clone();
        let current_report = current_report.clone();
        move |file| {
            files_handle.set(Some(file));
            current_report.set(None);
        }
    });
    let exercise_exists = use_state_eq(move || None::<bool>);

    {
        let c_exercise_exists = exercise_exists.clone();
        let url = format!("api/v1/check/{}", exercise);
        wasm_bindgen_futures::spawn_local(async move {
            let rep = crate::api::send_json_get_status(&url).await;
            match rep {
                Err(err) => {
                    log::error!("{err:?}");
                    c_exercise_exists.set(Some(false));
                }
                Ok(200) => c_exercise_exists.set(Some(true)),
                Ok(404) => c_exercise_exists.set(Some(false)),
                _ => {
                    c_exercise_exists.set(Some(false));
                }
            }
        });

        match exercise_exists.as_ref() {
            None => return html!(),
            Some(false) => {
                use crate::components::NotFoundPage;
                return html!(<NotFoundPage/>);
            }
            _ => {}
        }
    }

    if let Some(file) = files_handle.as_ref() {
        let file = file.clone();
        let name = file.name;
        let content = file.data;

        let handle = content.as_slice();
        let mut cursor = std::io::Cursor::new(handle);

        let json = smodel::json_from_sb3_stream(&mut cursor, Some(name));

        let body = json.map(|json| {
            let mut map = serde_json::Map::new();
            map.insert("program".into(), json);
            map.insert("exercise".into(), exercise.to_string().into());
            map.insert("agent".into(), "web".into());
            map.insert("session".into(), user_id.to_string().into());
            map
        });

        match body {
            Ok(json) => {
                let c_current_report = current_report.clone();
                if c_current_report.is_none() {
                    wasm_bindgen_futures::spawn_local(async move {
                        let rep = crate::api::send_json_post_json("api/v1/run", &json).await;
                        if rep.is_err() {
                            log::error!("{rep:?}");
                        }
                        c_current_report.set(Some(rep.map_err(|err| err.into_sharable())));
                    });
                }

                let done = |report: &Report| {
                    html!(
                        <>
                        <h1>{ "Willkommen zu Bast3St" }</h1>
                        <FileUpload file_selected={file_selected.clone()}/>
                        <ReportComponent report={report.clone()} class="report"/>
                        </>
                    )
                };
                let upload_error = |serr: &crate::api::SharableError| {
                    let msg = match serr {
                        SharableError::ReqConv(err) => html!(<>
                            <p>{"Ich habe es nicht geschafft, Ihre Abgabe in ein Format zu überführen, das ich hochladen kann. Das sollte niemals passieren."}</p>
                            <details>
                                <summary>{"Eventuell sehr langer, technischer Fehler"}</summary>
                                <code>{err}</code>
                            </details>
                        </>),
                        SharableError::RespError(err) => html!(<>
                            {"Ich konnte Ihre Abgabe nicht hochladen. Wenn Sie sicher mit dem Internet verbunden sind, stimmt vielleicht etwas mit dem Server nicht."}
                            <details>
                                <summary>{"Eventuell sehr langer, technischer Fehler"}</summary>
                                {err}
                            </details>
                        </>),
                        SharableError::NoJsonError { status, text, err } => match status {
                            413 => html!(<>
                            {"Es sieht so aus, als hätte Ihre Datei die erlaubte Maximalgröße für Abgaben überschritten. Diese Beschränkung gibt es nur, um sich for Angreifern zu schützen. Sie können im Forum nachfragen, damit diese Beschränkung höher gesetzt wird."}
                                <details>
                                    <summary>{"Eventuell sehr langer, technischer Fehler"}</summary>
                                    <ul>
                                        <li><code>{text}</code></li>
                                        <li><code>{err}</code></li>
                                    </ul>
                                </details>
                            </>),
                            422 => html!(<>
                                <p>{"Es tut mit sehr leid, aber Ihre Datei widerspricht meinem bisherigen Wissen über das Scratch-Dateiformat; das kannte ich bisher nur von KI-generierten Programmen. Wenn Sie die Datei dem Orga-Team schicken, kann dieser Fehler weiter untersucht werden."}</p>
                                <details>
                                    <summary>{"Eventuell sehr langer, technischer Fehler"}</summary>
                                    <ul>
                                        <li><code>{text}</code></li>
                                        <li><code>{err}</code></li>
                                    </ul>
                                </details>
                            </>),
                            _ => html!(<>
                            {"Ich kann nicht einordnen, was passiert ist. Ich glaube jedoch nicht, dass es an Ihrer Datei liegt. Am besten schreiben Sie in das Forum."}
                                <details>
                                    <summary>{"Eventuell sehr langer, technischer Fehler"}</summary>
                                    <ul>
                                        <li><code>{status}</code></li>
                                        <li><code>{text}</code></li>
                                        <li><code>{err}</code></li>
                                    </ul>
                                </details>
                            </>
                            ),
                        },
                    };

                    html!(
                        <>
                        <h1>{ "Willkommen zu Bast3St" }</h1>
                        <FileUpload file_selected={file_selected.clone()}/>
                        <div class="fatal-error-box">
                        <h2>{"Es ist ein Fehler aufgetreten"}</h2>
                        {msg}
                        </div>
                        </>
                    )
                };

                match &(*current_report) {
                    Some(Ok(report)) => done(report),
                    Some(Err(serr)) => upload_error(serr),
                    None => html!(
                        <>
                        <h1>{ "Willkommen zu Bast3St" }</h1>
                        <FileUpload {file_selected}/>
                        <p>{"Bitte warten..."}</p>
                        </>
                    ),
                }
            }
            Err(_error) => html!(
                <>
                <h1>{ "Willkommen zu Bast3St" }</h1>
                <FileUpload {file_selected}/>
                <p>{"Irgendetwas stimmt nicht mit der Datei, die sie hochgeladen haben. Ist das wirklich ein Scratch-Programm?"}</p>
                </>
            ),
        }
    } else {
        html!(
            <>
                <h1>{ "Willkommen zu Bast3St" }</h1>
                <FileUpload {file_selected}/>
                <InfoSection/>
            </>
        )
    }
}
