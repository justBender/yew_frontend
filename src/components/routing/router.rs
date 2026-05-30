use yew::{html, Html};
use yew_router::components::Redirect;
use yew_router::Routable;
use crate::components::{container_component::ContainerComponent, not_found::NotFoundComponent};
use crate::components::login::LoginComponent;

#[warn(unused_variables)]
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/home")]
    Home,
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/:path...")]
    CatchAll,
}

pub fn switch(route: Route) -> Html {
    match route {

        Route::Landing => html! {
            <ContainerComponent active={false}/>
        },

        Route::Home => html! {
            <ContainerComponent active={false}/>
        },

        Route::Login => html! {
            <LoginComponent />
        },
        
        Route::Register => html! {
            <ContainerComponent active={false}/>
        },

        Route::NotFound => html! { <NotFoundComponent /> },

        Route::CatchAll => html! {
            <Redirect<Route> to={Route::NotFound} />
        },
    }
}