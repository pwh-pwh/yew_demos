use yew::{function_component, Html, html};
use crate::Board;
// canonical
#[function_component]
pub fn App() ->Html {
    html!{
        <>
        <Board />
        </>
    }
}