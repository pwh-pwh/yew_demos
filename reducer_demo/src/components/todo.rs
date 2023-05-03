use yew::{AttrValue, Callback, ContextProvider, function_component, html, Html, Properties, use_memo, use_reducer, use_state, UseReducerHandle, UseStateHandle};
use crate::components::todoInput::TodoInput;
use std::rc::Rc;
use crate::components::todoList::TodoList;
use crate::store::reducer::TodoPropsVec;

#[derive(PartialEq, Properties,Clone)]
pub struct TodoProps {
    pub id:i64,
    pub text:AttrValue,
    pub isFinished:bool,
}

#[derive(Clone,PartialEq)]
pub struct ContextProps {
    pub todoReducerHandler: UseReducerHandle<TodoPropsVec>,
}

#[function_component]
pub fn Todo() -> Html {

    let todoReducer = use_reducer(TodoPropsVec::default);
    let ctx = ContextProps {
        todoReducerHandler: todoReducer,
    };

    html! {
        <ContextProvider<ContextProps> context={ctx}>
        <div class="container">
            <TodoInput/>
            <TodoList/>
        </div>
        </ContextProvider<ContextProps>>
    }
}