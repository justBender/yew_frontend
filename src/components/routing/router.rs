use crate::components::reset::Reset;
use crate::components::{
    blog::Blog, contact::Contact, container::Container, content::Content,
    context::auth_context::AuthContext, login::Login, not_found::NotFoundComponent,
    profile::Profile, send_reset::SendReset, signup::Signup, stack::Stack, wrapper::Wrapper,
};
use yew::{Callback, Html, html};
use yew_router::Routable;
use yew_router::components::Redirect;

#[warn(unused_variables)]
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/home")]
    Home,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/send-reset")]
    SendReset,
    #[at("/pass-reset")]
    Reset,
    #[at("/blog")]
    Blog,
    #[at("/contact")]
    Contact,
    #[at("/stack")]
    Stack,
    #[at("/profile")]
    Profile,
    #[at("/404")]
    NotFound,
    #[not_found]
    #[at("/:path...")]
    CatchAll,
}

pub fn switch(route: Route, callback: Callback<AuthContext>) -> Html {
    match route {
        Route::Landing => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Content image={"static/assets/BenderAndFerris_cutout.png"} />
                        </Wrapper>
                    </Container>
        },

        Route::Home => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Content image={"static/assets/BenderAndFerris_cutout.png"} />
                        </Wrapper>
                    </Container>
        },

        Route::Login => html! {
                   <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Login context_callback={callback.clone()} />
                        </Wrapper>
                   </Container>
        },

        Route::Signup => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Signup />
                        </Wrapper>
                    </Container>
        },

        Route::SendReset => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <SendReset />
                        </Wrapper>
                    </Container>
        },

        Route::Reset => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Reset />
                        </Wrapper>
                    </Container>
        },

        Route::Blog => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Blog image={"static/assets/BenderAndFerris_cutout.png"} />
                        </Wrapper>
                    </Container>
        },

        Route::Stack => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Stack />
                        </Wrapper>
                    </Container>
        },

        Route::Contact => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Contact />
                        </Wrapper>
                    </Container>
        },

        Route::Profile => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Profile context_callback={callback.clone()}/>
                        </Wrapper>
                    </Container>
        },

        Route::NotFound => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <NotFoundComponent />
                        </Wrapper>
                    </Container>
        },

        Route::CatchAll => html! {
                    <Container context_callback={callback.clone()}>
                        <Wrapper>
                            <Redirect<Route> to={Route::NotFound} />
                        </Wrapper>
                    </Container>
        },
    }
}
