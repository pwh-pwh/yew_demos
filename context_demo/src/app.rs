use yew::{function_component, Html, html};
use crate::components::todo::Todo;

#[function_component]
pub fn App() ->Html {

    html!{
        <div class="container">
            <Todo />
        </div>
    }
}