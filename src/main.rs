mod components;

use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Switch};
use crate::components::routing::router::{switch, Route};

#[derive(PartialEq, Clone)]
pub struct AppState {
    pub side_bar_active: bool,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
