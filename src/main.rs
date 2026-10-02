mod components;
pub mod services;
use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
