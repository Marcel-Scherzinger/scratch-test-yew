mod api;
mod components;
mod messages;
mod report;
mod routes;
mod testcase;
mod utils;

use derive_more::{Deref, From};
pub use routes::MainRoute;
use serde::{Deserialize, Serialize};
pub use utils::MaterialIcon;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_local_storage;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deref, From, Serialize, Deserialize)]
pub struct UserIdentifier(AttrValue);

#[function_component(App)]
fn app() -> Html {
    use routes::switch_main;
    use yew_router::prelude::*;

    let user_identifier_storage =
        use_local_storage::<UserIdentifier>("scratch-test-user-identifier".to_string());

    let user_identifier = if let Some(user) = &*user_identifier_storage {
        user.clone()
    } else {
        let id = UserIdentifier(uuid::Uuid::new_v4().to_string().into());
        user_identifier_storage.set(id.clone());
        id
    };

    let ctx = use_state(|| /* read_scheme().unwrap_or(Theme::Dark) */ Theme::Dark);
    {
        let document = gloo::utils::document();
        if let Some(html) = document.document_element() {
            html.set_attribute("data-theme", &ctx.to_string()).ok();
        }
    }
    const BASENAME: Option<&str> = std::option_env!("ROUTER_BASENAME");

    html!(
        <ContextProvider<UserIdentifier> context={user_identifier}>
        <ContextProvider<UseStateHandle<Theme>> context={ctx.clone()}>
             <BrowserRouter basename={BASENAME}>
                <Switch<MainRoute> render={switch_main} />
             </BrowserRouter>
        </ContextProvider<UseStateHandle<Theme>>>
        </ContextProvider<UserIdentifier>>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
