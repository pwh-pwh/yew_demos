use yew::{function_component, Html, html, Properties, props};
use yewdux::prelude::use_selector;
use crate::components::todoItem::TodoItem;
use crate::store::reducer_store::TodoPropsVec;



const todoListStyle:&str = "margin-top: 20px;";

#[function_component]
pub fn TodoList() -> Html {
    let todoList = use_selector(|todoVec:&TodoPropsVec|todoVec.0.clone());
    let todoListDom:Html = todoList.iter()
        .map(|item|html!{
            <TodoItem key={item.id} todo={item.clone()} />
        }).collect();
    html! {
        <div style={todoListStyle}>
            {todoListDom}
        </div>
    }
}