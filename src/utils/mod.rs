use yew::prelude::*;

#[derive(Properties, derive_more::PartialEq)]
pub struct MaterialIconProps {
    pub icon: AttrValue,
}
#[function_component(MaterialIcon)]
pub fn material_icon(MaterialIconProps { icon }: &MaterialIconProps) -> Html {
    html!(<span class="material-symbols-outlined">{icon}</span>)
}

#[derive(Properties, derive_more::PartialEq)]
pub struct LiteralProps {
    pub text: AttrValue,
    #[prop_or_default]
    pub classes: Classes,
}
#[function_component(Literal)]
pub fn literal_text(LiteralProps { text, classes }: &LiteralProps) -> Html {
    html!(<span class={classes!("literal-text", classes.clone())}>{text.replace(" ","␣")}</span>)
}
