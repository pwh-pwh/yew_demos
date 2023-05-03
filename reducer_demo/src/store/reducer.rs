use std::rc::Rc;
use yew::Reducible;
use crate::components::todo::TodoProps;

pub enum Action {
    Add(TodoProps),
    ChangeFinished(i64),
}

#[derive(PartialEq)]
pub struct TodoPropsVec(pub Vec<TodoProps>);

impl Default for TodoPropsVec {
    fn default() -> Self {
        Self(vec![])
    }
}



impl Reducible for TodoPropsVec {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let newVec = match action {
            //是否原来到值需要改变
            Action::Add(todo) => {
                let mut newV = self.0.clone();
                newV.push(todo);
                newV
            },
            Action::ChangeFinished(id) => {
                self.0.clone().into_iter()
                    .map(|mut item|{
                        if item.id == id {
                            item.isFinished = !item.isFinished;
                        }
                        item
                    }).collect::<Vec<TodoProps>>()
            },
        };
        Self(newVec).into()
    }
}