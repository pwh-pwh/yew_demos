mod app;
pub mod components;
pub use components::Square::Square;
pub use components::board::Board;

use app::App;
fn main() {
    yew::Renderer::<App>::new().render();
}
