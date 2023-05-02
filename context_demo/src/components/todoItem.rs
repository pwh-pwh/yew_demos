use yew::{Callback, function_component, html, Html, Properties, use_context};
use crate::components::todo::{ContextProps, TodoProps};

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: TodoProps,
}


#[function_component]
pub fn TodoItem(TodoItemProps{todo}: &TodoItemProps) -> Html {
    let todoId = todo.id;
    let context = use_context::<ContextProps>().expect("转换失败");
    let changeHandler = {
        let changeTodo = context.changeTodo.clone();
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