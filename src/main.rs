mod components;
mod messages;
mod routes;
mod testcase;
mod utils;

pub use routes::MainRoute;
pub use utils::MaterialIcon;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    use routes::switch_main;
    use yew_router::prelude::*;
    html!(
         <BrowserRouter>
            <Switch<MainRoute> render={switch_main} />
         </BrowserRouter>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
