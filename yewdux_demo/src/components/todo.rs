use yew::{AttrValue, Callback, function_component, html, Html, Properties, use_state, UseStateHandle};
use crate::components::todoInput::TodoInput;
use crate::components::todoList::TodoList;
#[derive(PartialEq, Properties,Clone,Eq)]
pub struct TodoProps {
    pub id:i64,
    pub text:AttrValue,
    pub isFinished:bool,
}

#[function_component]
pub fn Todo() -> Html {

    html! {
        <div class="container">
            <TodoInput/>
            <TodoList/>
        </div>
    }
}