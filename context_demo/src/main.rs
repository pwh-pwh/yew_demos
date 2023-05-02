use crate::app::App;

mod app;
mod components;

fn main() {
    yew::Renderer::<App>::new().render();
}
