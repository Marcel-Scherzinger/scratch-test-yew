mod api;
mod components;
mod messages;
mod report;
mod routes;
mod testcase;
mod utils;

pub use routes::MainRoute;
pub use utils::MaterialIcon;
use web_sys::window;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, derive_more::Display)]
enum Theme {
    #[display("dark")]
    Dark,
    #[display("light")]
    Light,
}

fn read_scheme() -> Option<Theme> {
    let mql = window()?
        .match_media("(prefers-color-scheme: dark)")
        .ok()??;
    Some(if mql.matches() {
        Theme::Dark
    } else {
        Theme::Light
    })
}

#[function_component(App)]
fn app() -> Html {
    use routes::switch_main;
    use yew_router::prelude::*;

    let ctx = use_state(|| read_scheme().unwrap_or(Theme::Dark));
    {
        let document = gloo::utils::document();
        if let Some(html) = document.document_element() {
            html.set_attribute("data-theme", &ctx.to_string()).ok();
        }
    }

    html!(
        <ContextProvider<UseStateHandle<Theme>> context={ctx.clone()}>
         <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
         </BrowserRouter>
        </ContextProvider<UseStateHandle<Theme>>>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
