use yew::prelude::*;

#[derive(Properties, derive_more::PartialEq)]
pub struct MaterialIconProps {
    pub icon: AttrValue,
}
#[function_component(MaterialIcon)]
pub fn material_icon(MaterialIconProps { icon }: &MaterialIconProps) -> Html {
    html!(<span class="material-symbols-outlined">{icon}</span>)
}
