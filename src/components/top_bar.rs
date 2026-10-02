use crate::components::context::auth_context::AuthContext;
use crate::components::routing::router::Route;
use web_sys::MouseEvent;
use yew::{Callback, Component, Context, ContextHandle, Html, Properties, html};
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;

#[warn(unused_variables)]
pub struct TopBar {
    pub auth_context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq)]
pub struct TopBarProps {
    pub toggle: Callback<MouseEvent>,
    pub context_callback: Callback<AuthContext>,
}

pub enum TopBarMsg {
    AuthUpdate(AuthContext),
    Logout,
}

impl Component for TopBar {
    type Message = TopBarMsg;
    type Properties = TopBarProps;
    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, listener) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(Self::Message::AuthUpdate))
            .expect("TopBarMsg::create failed");
        Self {
            auth_context,
            _listener: listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let ctx_copy = ctx.props().clone();
        let link_copy = ctx.link().clone();
        match msg {
            Self::Message::AuthUpdate(auth_context) => {
                self.auth_context.state = auth_context.state;
                dbg!(
                    &self.auth_context.state,
                    &<Option<String> as Clone>::clone(&self.auth_context.state.name)
                        .unwrap_or_default()
                );
                true
            }
            Self::Message::Logout => {
                wasm_bindgen_futures::spawn_local(async move {
                    let context = AuthContext::logout().await.unwrap_or_default();
                    link_copy.send_message(Self::Message::AuthUpdate(context.clone()));
                    ctx_copy.context_callback.emit(context);
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.props().toggle.clone();
        let logout = ctx.link().callback(|_| TopBarMsg::Logout);
        html! {
            <>
                <div class="top-bar">
                    <div class="bar-el">
                        <button class="burger-button" onclick={onclick}>
                            <Icon
                                data={IconData::SIMPLE_ICONS_RUST}
                                width={"1.5rem".to_owned()}
                                height={"1.5rem".to_owned()}
                            />
                        </button>
                    </div>
                    <div class="bar-menu">
                        <div class="menu-el">
                            <Link<Route> to={Route::Home}>
                                <Icon
                                    data={IconData::HEROICONS_SOLID_HOME}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                        <div class="menu-el">
                            <Link<Route> to={Route::Contact}>
                                <Icon
                                    data={IconData::BOOTSTRAP_ENVELOPE_AT}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                        <div class="menu-el">
                            <Link<Route> to={Route::Stack}>
                                <Icon
                                    data={IconData::BOOTSTRAP_BRACES_ASTERISK}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                        <div class="menu-el">
                            <Link<Route> to={Route::Blog}>
                                <Icon
                                    data={IconData::BOOTSTRAP_QUOTE}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                        {
                            if self.auth_context.state.logged_in {
                                html! {
                                    <>
                                        <div class="menu-el" >
                                            <button class="logout-button" type="button" onclick={logout}>
                                                <Icon
                                                    data={IconData::HEROICONS_SOLID_ARROW_RIGHT_ON_RECTANGLE}
                                                    width={"1.5rem".to_owned()}
                                                    height={"1.5rem".to_owned()}
                                                />
                                            </button>
                                        </div>
                                        <div class="menu-el">
                                            <Link<Route> to={Route::Profile}>
                                                <Icon
                                                    data={IconData::BOOTSTRAP_PEOPLE}
                                                    width={"1.5rem".to_owned()}
                                                    height={"1.5rem".to_owned()}
                                                />
                                            </Link<Route>>
                                        </div>
                                        <div class="menu-el">
                                            <a href="https://github.com/justbender/" target="_blank" rel="noopener noreferrer" role="link">
                                                <Icon
                                                    data={IconData::BOOTSTRAP_GIT}
                                                    width={"1.2rem".to_owned()}
                                                    height={"1.2rem".to_owned()}
                                                />
                                            </a>
                                        </div>
                                    </>
                                }
                            } else {
                                html!{
                                <>
                                    <div class="menu-el">
                                        <Link<Route> to={Route::Login}>
                                            <Icon
                                                data={IconData::HEROICONS_SOLID_ARROW_LEFT_ON_RECTANGLE}
                                                width={"1.5rem".to_owned()}
                                                height={"1.5rem".to_owned()}
                                            />
                                        </Link<Route>>
                                    </div>
                                    <div class="menu-el">
                                        <Link<Route> to={Route::Signup}>
                                            <Icon
                                                data={IconData::HEROICONS_SOLID_PENCIL_SQUARE}
                                                width={"1.5rem".to_owned()}
                                                height={"1.5rem".to_owned()}
                                            />
                                        </Link<Route>>
                                    </div>
                                </>
                                }
                            }
                        }
                    </div>
                    /*<hr class="hr-line"/>*/
                </div>
            </>
        }
    }
}
