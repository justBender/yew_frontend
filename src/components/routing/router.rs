use yew::{html, Html};
use yew_router::components::Redirect;
use yew_router::Routable;
use crate::components::{container_component::ContainerComponent, not_found::NotFoundComponent};

#[warn(unused_variables)]
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    LandingPage,
    #[at("/login")]
    LoginPage,
    #[at("/home")]
    Home,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/:path...")]
    CatchAll,
}

pub fn switch(route: Route) -> Html {
    match route {

        Route::LandingPage => html! {
            <ContainerComponent active={false}/>
        },

        Route::Home => html! {
            <ContainerComponent active={false}/>
        },

        Route::LoginPage => html! {
            <ContainerComponent active={false}/>
        },

        Route::NotFound => html! { <NotFoundComponent /> },

        Route::CatchAll => html!{
            <Redirect<Route> to={Route::NotFound} />
        },
    }
}