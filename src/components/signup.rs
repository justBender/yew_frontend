use crate::components::context::auth_context::AuthContext;
use crate::components::routing::router::Route;
use crate::services::signup::SignupRequest;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconData};
use yew_router::components::Redirect;

pub struct Signup {
    pub name: String,
    pub email: String,
    pub password: String,
    pub password_repeat: String,
    pub roles: Vec<String>,
    pub message: String,
}

pub enum SignupMsg {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdatePasswordRepeat(String),
    Submit,
    SignupFinished(Result<String, String>),
}

impl Component for Signup {
    type Message = SignupMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            password_repeat: "".to_string(),
            roles: vec![],
            message: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SignupMsg::UpdateUsername(v) => {
                self.name = v;
                true
            }

            SignupMsg::UpdateEmail(v) => {
                self.email = v;
                true
            }

            SignupMsg::UpdatePassword(v) => {
                self.password = v;
                true
            }

            SignupMsg::UpdatePasswordRepeat(v) => {
                self.password_repeat = v;
                true
            }

            SignupMsg::Submit => {
                let mut req = SignupRequest {
                    name: self.name.clone(),
                    email: self.email.clone(),
                    password: self.password.clone(),
                    roles: self.roles.clone(),
                };
                if req.roles.is_empty() {
                    req.roles = vec!["User".to_string()];
                }
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let auth = AuthContext::signup(req).await;
                    link.send_message_batch(vec![SignupMsg::SignupFinished(auth)]);
                });
                true
            }
            SignupMsg::SignupFinished(result) => {
                match result {
                    Ok(message) => {
                        self.message = message;
                    }
                    Err(error) => {
                        println!("login failed: {}", error);
                        self.message = error.to_string();
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let password_match = !self.password.is_empty() && self.password == self.password_repeat;
        let submit_disabled = !password_match;
        html! {
            <div class="signup-form-container">
            {
                if !self.email.is_empty() {
                    html! {
                        <Redirect<Route> to={Route::Home}/>
                    }
                } else {
                    html! {}
                }
            }
                <form class="signup-form">
                    <div class="signup-input-box">
                       <div class="email-icon">
                            <Icon
                                    data={IconData::BOOTSTRAP_PERSON_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                            />
                       </div>
                       <input
                            required=true
                            name="name"
                            class="signup-input"
                            type="text"
                            placeholder="username"
                            autocomplete="on"
                            value={self.name.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                SignupMsg::UpdateUsername(input.value())
                            })}
                        />
                    </div>
                    <div class="signup-input-box">
                        <div class="password-icon">
                            <Icon
                                    data={IconData::BOOTSTRAP_AT}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                            />
                        </div>
                        <input
                            required=true
                            name="email"
                            class="signup-input"
                            type="email"
                            placeholder="email"
                            autocomplete="on"
                            value={self.email.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                SignupMsg::UpdateEmail(input.value())
                            })}
                        />
                    </div>
                    <div class="signup-input-box">
                        <div class="password-icon">
                            <Icon
                                    data={IconData::BOOTSTRAP_KEY_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                            />
                        </div>
                        <input
                            required=true
                            name="password"
                            class="signup-input"
                            type="password"
                            placeholder="password"
                            autocomplete="on"
                            value={self.password.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                SignupMsg::UpdatePassword(input.value())
                            })}
                        />
                    </div>
                    <div class="signup-input-box">
                        <div class="password-icon">
                            <Icon
                                    data={IconData::BOOTSTRAP_KEY}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                            />
                        </div>
                        <input
                            required=true
                            name="password-repeat"
                            class="signup-input"
                            type="password"
                            placeholder="repeat password"
                            autocomplete="on"
                            value={self.password_repeat.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                SignupMsg::UpdatePasswordRepeat(input.value())
                            })}
                        />
                    </div>
                    {
                        if password_match {
                            html! {
                                <div class="passmatch">
                                    {"passwords match"}
                                </div>
                            }
                        } else {
                            html! {
                                <div class="passnomatch">
                                    {"passwords must match"}
                                </div>
                            }
                        }
                    }
                    <button
                        class="signup-submit-button"
                        type="button"
                        disabled={submit_disabled}
                        onclick={ctx.link().callback(|_| SignupMsg::Submit)}
                    >
                        {"signup"}
                    </button>
                </form>
            </div>
        }
    }
}
