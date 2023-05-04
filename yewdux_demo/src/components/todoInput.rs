use js_sys::Date;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Html, use_effect, use_node_ref};
use gloo_console::log;
use crate::components::todo::TodoProps;
use yewdux::prelude::*;
use crate::store::reducer_store::TodoAction::Add;
use crate::store::reducer_store::TodoPropsVec;



#[function_component]
pub fn TodoInput() -> Html {
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        use_effect(move||{
            let input_ref = input_ref.cast::<HtmlInputElement>().expect("can not fc");
            _ = input_ref.focus();
        });
    }


    let dispatch = Dispatch::<TodoPropsVec>::new();
    let addTodoHandler = {
        let input_ref = input_ref.clone();
        // log!(input_ref);
        log!("my log");

//误区，组件还没渲染
        /*let mut input = input_ref.cast::<HtmlInputElement>().expect("can not fc");
        let text = input.value();*/
         move|_| {
             let mut input = input_ref.cast::<HtmlInputElement>().expect("can not fc");
             dispatch.apply(Add(TodoProps{
                 id: Date::new_0().get_time() as i64,
                 text: input.value().into(),
                 isFinished: false,
             }));
             input.set_value("");
        }
    };
    html! {
        <>
        <div class="input-group mb-3">
            <input type="text" placeholder="请输入代办事项" ref={input_ref.clone()}/>
            <button class="form-control" onclick={addTodoHandler}>{"添加"}</button>
        </div>
        </>
    }
}