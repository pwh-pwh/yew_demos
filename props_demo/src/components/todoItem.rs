use yew::{Callback, function_component, html, Html, Properties};
use crate::components::todo::TodoProps;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: TodoProps,
    pub changeTodo: Callback<i64>,
}


#[function_component]
pub fn TodoItem(TodoItemProps{todo,changeTodo}: &TodoItemProps) -> Html {
    let todoId = todo.id;
    let changeHandler = {
        let changeTodo = changeTodo.clone();
        move|_| changeTodo.emit(todoId)
    };
    let spanStyle = if todo.isFinished { "line-through" } else {"none"};
    html! {
        <div >
            <input type="checkbox" checked={todo.isFinished} onchange={changeHandler}/>
            <span style={spanStyle} class="checkable">{todo.text.clone()}</span>
        </div>
    }
}