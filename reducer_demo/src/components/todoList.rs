use yew::{Callback, function_component, Html, html, Properties, props, use_context};
use yew::context::_ContextProviderProps::context;
use crate::components::todo::{ContextProps, TodoProps};
use crate::components::todoItem::TodoItem;


const todoListStyle:&str = "margin-top: 20px;";

#[function_component]
pub fn TodoList() -> Html {
    let ContextProps{todoReducerHandler} = use_context::<ContextProps>().expect("转换失败");
    let todoList = todoReducerHandler.0.clone();
    let todoListDom:Html = todoList.into_iter()
        .map(|item|html!{
            <TodoItem key={item.id} todo={item.clone()} />
        }).collect();
    html! {
        <div style={todoListStyle}>
            {todoListDom}
        </div>
    }
}