use crate::components::context::auth_context::AuthContext;
use crate::components::routing::router::Route;
use crate::services::login::LoginRequest;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, InputEvent, MouseEvent};
use yew::{Callback, Component, Context, ContextHandle, Html, Properties, html};
use yew_icons::{Icon, IconData};
use yew_router::prelude::Redirect;

#[warn(unused_variables)]
pub struct Login {
    pub email: String,
    pub password: String,
    pub response: String,
    pub auth_context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

pub enum LoginMsg {
    EmailChanged(String),
    PasswordChanged(String),
    Submit,
    LoginFinished(Result<AuthContext, String>),
    AuthUpdate(AuthContext),
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct LoginProps {
    pub context_callback: Callback<AuthContext>,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = LoginProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, handle) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(LoginMsg::AuthUpdate))
            .expect("missing auth context");

        Self {
            email: "".to_string(),
            password: "".to_string(),
            response: "".to_string(),
            auth_context: auth_context.clone(),
            _listener: handle,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LoginMsg::Submit => {
                let req = LoginRequest {
                    email: self.email.clone(),
                    password: self.password.clone(),
                };

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let auth = AuthContext::login(req).await;
                    link.send_message_batch(vec![LoginMsg::LoginFinished(auth)]);
                });
                false
            }
            LoginMsg::LoginFinished(result) => {
                match result {
                    Ok(auth) => {
                        ctx.props().context_callback.emit(auth.clone());
                        self.auth_context = auth;
                    }
                    Err(err) => {
                        self.response = err;
                    }
                }

                true
            }
            LoginMsg::EmailChanged(v) => {
                self.email = v;
                true
            }
            LoginMsg::PasswordChanged(v) => {
                self.password = v;
                true
            }
            LoginMsg::AuthUpdate(auth_context) => {
                self.auth_context = auth_context.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let username = self.email.clone();
        let password = self.password.clone();
        html! {
            <>
                {
                    if self.auth_context.state.logged_in {
                        html! {
                            <Redirect<Route> to={Route::Home}/>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="login-form-container">
                    <form class="login-form" method="post">
                        <div class="email">
                            <div class="email-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_PERSON_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                                />
                            </div>
                            <input
                                class="email-input"
                                autocomplete="on"
                                name="email"
                                placeholder="email"
                                type="email"
                                oninput={ctx.link().callback(|event: InputEvent| {
                                        LoginMsg::EmailChanged(
                                            event
                                                .target()
                                                .expect("Event should have a target when dispatched")
                                                .unchecked_into::<HtmlInputElement>()
                                                .value())
                                })}
                                value={username.clone()}
                            />
                        </div>
                        <div class="password">
                            <div class="password-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_KEY_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                                />
                            </div>
                            <input
                                class="password-input"
                                autocomplete="on"
                                name="password"
                                placeholder="password"
                                type="password"
                                oninput={ctx.link().callback(|event: InputEvent| {
                                        LoginMsg::PasswordChanged(
                                            event
                                                .target()
                                                .expect("Event should have a target when dispatched")
                                                .unchecked_into::<HtmlInputElement>()
                                                .value())
                                })}
                                value={password.clone()}
                            />
                        </div>
                        <div class="reglog">
                            <button
                                type="button"
                                class="submit-button"
                                onclick={ctx.link().callback(move |e:MouseEvent| {
                                    e.prevent_default();
                                    LoginMsg::Submit
                                })}
                            >
                                {"login"}
                            </button>
                        </div>
                        <div class="pass-reset-links">
                            <div class="pass-reset-link">
                                <a href="send-reset" class="pass-reset">{"password-reset"}</a>
                            </div>
                            <div class="signup-link">
                                <a href="signup">{"or sign up?"}</a>
                            </div>
                        </div>
                    </form>
                </div>
            </>
        }
    }
}
