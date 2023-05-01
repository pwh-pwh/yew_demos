use js_sys::Date;
use web_sys::HtmlInputElement;
use yew::{AttrValue, Callback, function_component, html, Html, Properties, TargetCast, use_state};
use yew::html::onchange;
use crate::components::todo::TodoProps;



#[derive(PartialEq, Properties)]
pub struct TodoInputProps {
    pub addTodo: Callback<TodoProps>,
}

#[function_component]
pub fn TodoInput(props: &TodoInputProps) -> Html {
    let textState = use_state(String::default);
    let changeTextHandler = {
        let textState = textState.clone();
        Callback::from(move |e:onchange::Event| {
            textState.set(e.target_unchecked_into::<HtmlInputElement>().value())
        })
    };
    let addTodoHandler = {
        let textState = textState.clone();
        let text:AttrValue = (*textState).clone().into();
        let addTodo = props.addTodo.clone();
         move|_| {
            addTodo.emit(TodoProps{
                id: Date::new_0().get_time() as i64,
                text: text.clone(),
                isFinished: false,
            });
            textState.set("".into());
        }
    };
    let text = (*textState).clone();
    html! {
        <>
        <div class="input-group mb-3">
            <input type="text" placeholder="请输入代办事项" onchange={changeTextHandler} value={text}/>
            <button class="form-control" onclick={addTodoHandler}>{"添加"}</button>
        </div>
        </>
    }
}