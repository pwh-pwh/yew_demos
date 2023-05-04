use yew::{Callback, function_component, html, Html, Properties};
use yewdux::prelude::Dispatch;
use crate::components::todo::TodoProps;
use crate::store::reducer_store::TodoAction::ChangeTodo;
use crate::store::reducer_store::TodoPropsVec;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: TodoProps,
}


#[function_component]
pub fn TodoItem(TodoItemProps{todo}: &TodoItemProps) -> Html {
    let todoId = todo.id;
    let dispatch = Dispatch::<TodoPropsVec>::new();
    let changeHandler = dispatch.apply_callback(move|_|ChangeTodo(todoId));
    let spanStyle = if todo.isFinished { "line-through" } else {"none"};
    html! {
        <div >
            <input type="checkbox" checked={todo.isFinished} onchange={changeHandler}/>
            <span style={spanStyle} class="checkable">{todo.text.clone()}</span>
        </div>
    }
}