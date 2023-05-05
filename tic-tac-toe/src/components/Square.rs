use gloo_console::log;
use yew::{AttrValue, Callback, function_component, html, Html, MouseEvent, Properties, use_state};

#[derive(PartialEq,Properties)]
pub struct SquareProps {
    pub value:AttrValue,
    pub onSquareClick:Callback<MouseEvent>,
}

#[function_component]
pub fn Square(props:&SquareProps) -> Html {
    html! {
            <button class="square" onclick={props.onSquareClick.clone()}>
                {props.value.clone()}
            </button>
    }
}