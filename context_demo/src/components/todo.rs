use yew::{AttrValue, Callback, ContextProvider, function_component, html, Html, Properties, use_memo, use_state, UseStateHandle};
use crate::components::todoInput::TodoInput;
use std::rc::Rc;
use crate::components::todoList::TodoList;

#[derive(PartialEq, Properties,Clone)]
pub struct TodoProps {
    pub id:i64,
    pub text:AttrValue,
    pub isFinished:bool,
}

#[derive(Clone,PartialEq)]
pub struct ContextProps {
    pub todoList: Vec<TodoProps>,
    pub changeTodo: Callback<i64>,
    pub addTodo: Callback<TodoProps>,
}

#[function_component]
pub fn Todo() -> Html {
    let todoListState:UseStateHandle<Vec<TodoProps>> = use_state(|| Vec::new());
    let changeTodo = {
        let todoList = (*todoListState).clone();
        let todoListState = todoListState.clone();
        move|id:i64| {
            let newTodoList:Vec<TodoProps> = todoList.clone().into_iter()
                .map(|mut item| {
                    if item.id == id {
                        item.isFinished = !item.isFinished;
                    }
                    item
                }).collect();
            todoListState.set(newTodoList);
        }
    };

    let addTodo = {
        let mut todoList = (*todoListState).clone();
        let todoListState = todoListState.clone();
        Callback::from(move|todo:TodoProps|{
            let mut todoList = todoList.clone();
            todoList.push(todo);
            todoListState.set(todoList);
        })
    };
    let todoList = (*todoListState).clone();
    let ctx = ContextProps {
        todoList: (*todoListState).clone(),
        changeTodo: Callback::from(changeTodo),
        addTodo,
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