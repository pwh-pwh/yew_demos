use yew::{Callback, function_component, html, Html, Properties, use_context};
use yew::context::_ContextProviderProps::context;
use crate::components::todo::{ContextProps, TodoProps};
use crate::store::reducer::Action;

#[derive(PartialEq, Properties)]
pub struct TodoItemProps {
    pub todo: TodoProps,
}


#[function_component]
pub fn TodoItem(TodoItemProps{todo}: &TodoItemProps) -> Html {
    let todoId = todo.id;
    let ContextProps{todoReducerHandler} = use_context::<ContextProps>().expect("转换失败");
    let changeHandler = {
        let changeTodo = todoReducerHandler.clone();
        move|_| changeTodo.dispatch(Action::ChangeFinished(todoId))
    };
    let spanStyle = if todo.isFinished { "line-through" } else {"none"};
    html! {
        <div >
            <input type="checkbox" checked={todo.isFinished} onchange={changeHandler}/>
            <span style={spanStyle} class="checkable">{todo.text.clone()}</span>
        </div>
    }
}