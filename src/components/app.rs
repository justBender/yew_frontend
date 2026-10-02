use crate::components::context::auth_context::AuthContext;
use crate::components::routing::router::{Route, switch};
use yew::{Children, Component, Context, ContextProvider, Html, Properties, html};
use yew_router::{BrowserRouter, Switch};

#[derive(Clone, PartialEq, Default)]
pub struct App {
    pub side_bar_active: bool,
    pub auth_context: AuthContext,
}

#[derive(Properties, PartialEq, Clone, Default)]
pub struct AppProps {
    pub side_bar_active: bool,
    #[prop_or_default]
    pub children: Children,
    pub auth_context: AuthContext,
}

pub enum AppMsg {
    AuthUpdate(AuthContext),
    UpdateResponse(Result<AuthContext, String>),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = AppProps;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        wasm_bindgen_futures::spawn_local(async move {
            let auth = AuthContext::refresh().await;
            link.send_message(AppMsg::UpdateResponse(auth));
        });
        Self {
            side_bar_active: false,
            auth_context: AuthContext::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::AuthUpdate(auth) => {
                self.auth_context = auth;
                true
            }
            AppMsg::UpdateResponse(Ok(auth)) => {
                self.auth_context = auth;
                true
            }
            AppMsg::UpdateResponse(Err(err)) => {
                println!("{}", err);
                self.auth_context = AuthContext::default();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let set_auth = ctx.link().callback(|auth| AppMsg::AuthUpdate(auth));
        html! {
            <>
                <ContextProvider<AuthContext> context={self.auth_context.clone()}>
                    <BrowserRouter>
                        <Switch<Route> render={move |route| switch(route, set_auth.clone())} />
                    </BrowserRouter>
                </ContextProvider<AuthContext>>
            </>
        }
    }
}
