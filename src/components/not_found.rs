use yew::prelude::*;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html!(
        <div>
            <div>
                <div>
                <h1 style={"margin: auto; text-align: center;"}>{"Diese Seite existiert nicht"}</h1>
            </div>
            </div>
        </div>
    )
}
