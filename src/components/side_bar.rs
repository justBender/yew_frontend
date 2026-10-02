use crate::components::context::auth_context::AuthContext;
use crate::components::routing::router::Route;
use yew::{Callback, Component, Context, ContextHandle, Html, Properties, html};
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

#[warn(unused_variables)]
pub struct SideBar {
    pub auth_context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq)]
pub struct SideBarProps {
    pub active: bool,
    pub context_callback: Callback<AuthContext>,
}

pub enum SideBarMsg {
    AuthUpdate(AuthContext),
}

impl Component for SideBar {
    type Message = SideBarMsg;
    type Properties = SideBarProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, listener) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(Self::Message::AuthUpdate))
            .expect("missing auth context");
        Self {
            auth_context,
            _listener: listener,
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::AuthUpdate(auth_context) => {
                self.auth_context = auth_context;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let active = if ctx.props().active {
            "side-bar"
        } else {
            "hidden"
        };
        html! {
            <>
                <div class={active}>
                    {
                        if self.auth_context.state.logged_in {
                            let state = self.auth_context.state.clone();
                            html!{
                                <>
                                    <div class="side-bar-el">
                                       <Link<Route> classes="profile-link" to={Route::Profile}>
                                            <Icon
                                                data={IconData::BOOTSTRAP_PEOPLE}
                                                width={"1.5rem".to_owned()}
                                                height={"1.5rem".to_owned()}
                                                class={"side-bar-icon"}
                                            />
                                            <div class="side-bar-el-label">{ state.name.as_ref().unwrap() }</div>
                                       </Link<Route>>
                                    </div>
                                    <div class="side-bar-el">
                                       <Link<Route> classes="profile-link" to={Route::Profile}>
                                            <Icon
                                                data={IconData::BOOTSTRAP_GIT}
                                                width={"1.5rem".to_owned()}
                                                height={"1.5rem".to_owned()}
                                                class={"side-bar-icon"}
                                            />
                                            <div class="side-bar-el-label">{"git"}</div>
                                       </Link<Route>>
                                    </div>
                                </>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <div class="side-bar-el">
                        <Link<Route> classes="profile-link" to={Route::Contact}>
                            <Icon
                                data={IconData::BOOTSTRAP_ENVELOPE_AT}
                                width={"1.5rem".to_owned()}
                                height={"1.5rem".to_owned()}
                                class={"side-bar-icon"}
                            />
                            <div class="side-bar-el-label">{"contact"}</div>
                        </Link<Route>>
                    </div>
                    <div class="side-bar-el">
                        <Link<Route> classes="profile-link" to={Route::Stack}>
                            <Icon
                                data={IconData::BOOTSTRAP_BRACES_ASTERISK}
                                width={"1.5rem".to_owned()}
                                height={"1.5rem".to_owned()}
                                class={"side-bar-icon"}
                            />
                            <div class="side-bar-el-label">{"stack"}</div>
                        </Link<Route>>
                    </div>
                    <div class="side-bar-el">
                        <Link<Route> classes="profile-link" to={Route::Blog}>
                            <Icon
                                data={IconData::BOOTSTRAP_QUOTE}
                                width={"1.5rem".to_owned()}
                                height={"1.5rem".to_owned()}
                                class={"side-bar-icon"}
                            />
                            <div class="side-bar-el-label">{"blog"}</div>
                        </Link<Route>>
                    </div>
                </div>
            </>
        }
    }
}
