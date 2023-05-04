use std::rc::Rc;
use crate::components::todo::TodoProps;
use yewdux::prelude::*;

#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct TodoPropsVec(pub Vec<TodoProps>);

pub enum TodoAction {
    Add(TodoProps),
    ChangeTodo(i64),
}

impl Reducer<TodoPropsVec> for TodoAction {
    fn apply(self, mut todoPV: Rc<TodoPropsVec>) -> Rc<TodoPropsVec> {
        let state = Rc::make_mut(  &mut todoPV);
        match self {
            TodoAction::Add(todo) => {
                state.0.push(todo);
            },
            TodoAction::ChangeTodo(id) => {
                state.0.iter_mut().for_each(|todo| {
                    if todo.id == id {
                        todo.isFinished = !todo.isFinished;
                    }
                });
            },
        };
        todoPV
    }
}