use sreport::report::Report;
use yew::prelude::*;

use crate::{
    UserIdentifier,
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
    let file_selected = Callback::from({
        let files_handle = files_handle.clone();
        move |file| files_handle.set(Some(file))
    });
    let current_report = use_state_eq(move || None::<Report>);
    let exercise_exists = use_state_eq(move || None::<bool>);

    {
        let c_exercise_exists = exercise_exists.clone();
        let url = format!("api/v1/check/{}", exercise);
        wasm_bindgen_futures::spawn_local(async move {
            let rep = crate::api::send_json_get_status(&url).await;
            match rep {
                Err(err) => log::error!("{err:?}"),
                Ok(200) => c_exercise_exists.set(Some(true)),
                Ok(404) => c_exercise_exists.set(Some(false)),
                _ => {}
            }
        });
    }
    if exercise_exists.is_none_or(|x| !x) {
        use crate::components::NotFoundPage;
        return html!(<NotFoundPage/>);
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
                wasm_bindgen_futures::spawn_local(async move {
                    let rep = crate::api::send_json_post_json("api/v1/run", json).await;
                    if rep.is_err() {
                        log::error!("{rep:?}");
                    }
                    c_current_report.set(rep.ok());
                });
                if let Some(report) = &(*current_report) {
                    html!(
                        <>
                        <h1>{ "Willkommen zu Bast3St" }</h1>
                        <FileUpload {file_selected}/>
                        <ReportComponent report={report.clone()}/>
                        </>
                    )
                } else {
                    html!(
                        <>
                        <h1>{ "Willkommen zu Bast3St" }</h1>
                        <FileUpload {file_selected}/>
                        <p>{"Ich konnte Ihre Abgabe nicht hochladen. Sind Sie mit dem Internet verbunden?"}</p>
                        </>
                    )
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
            </>
        )
    }
}
