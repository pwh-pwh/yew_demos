use crate::app::App;

mod app;
mod components;
mod store;

fn main() {
    yew::Renderer::<App>::new().render();
}
