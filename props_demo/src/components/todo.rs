use yew::{AttrValue, Callback, function_component, html, Html, Properties, use_state, UseStateHandle};
use crate::components::todoInput::TodoInput;
use crate::components::todoList::TodoList;
#[derive(PartialEq, Properties,Clone)]
pub struct TodoProps {
    pub id:i64,
    pub text:AttrValue,
    pub isFinished:bool,
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
        move|todo: TodoProps| {
            let mut todoList = todoList.clone();
            todoList.push(todo);
            todoListState.set(todoList.clone());
        }
    };
    let todoList = (*todoListState).clone();
    html! {
        <div class="container">
            <TodoInput {addTodo}/>
            <TodoList todoList={todoList} changeTodo={changeTodo}/>
        </div>
    }
}