use yew::{Callback, function_component, Html, html, Properties};
use crate::components::todo::TodoProps;
use crate::components::todoItem::TodoItem;

#[derive(PartialEq, Properties)]
pub struct TodoListProps {
    pub todoList: Vec<TodoProps>,
    pub changeTodo: Callback<i64>
}

const todoListStyle:&str = "margin-top: 20px;";

#[function_component]
pub fn TodoList(props: &TodoListProps) -> Html {
    let todoList = props.todoList.clone();
    let todoListDom:Html = todoList.into_iter()
        .map(|item|html!{
            <TodoItem key={item.id} todo={item.clone()} changeTodo={props.changeTodo.clone()}/>
        }).collect();
    html! {
        <div style={todoListStyle}>
            {todoListDom}
        </div>
    }
}