use crate::components::context::auth_context::AuthContext;
use crate::services::reset::ResetRequest;
use web_sys::{HtmlInputElement, InputEvent, window};
use yew::{Component, Context, Html, TargetCast, html};
use yew_icons::{Icon, IconData};

#[warn(unused_variables)]
pub struct Reset {
    pub password: String,
    pub password_repeat: String,
    pub token: String,
    pub response: String,
}

pub enum ResetMsg {
    PasswordChanged(String),
    PasswordRepeatChanged(String),
    Submit,
    ResetFinished(Result<String, String>),
}

impl Component for Reset {
    type Message = ResetMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let token = window()
            .and_then(|w| Some(w.location().search().ok().unwrap_or_default()))
            .and_then(|search| {
                // search is like "?token=abc" or "" (no query)
                let search = search.trim_start_matches('?');
                let mut id = None;
                for (k, v) in form_urlencoded::parse(search.as_bytes()).into_owned() {
                    if k == "token" {
                        id = Some(v.to_owned());
                        break;
                    }
                }
                id
            });
        let mut token_cpy = token.clone();
        if token_cpy.is_none() {
            token_cpy = Some("".to_string())
        }
        Self {
            password: "".to_string(),
            password_repeat: "".to_string(),
            token: token_cpy.unwrap(),
            response: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            ResetMsg::Submit => {
                let req = ResetRequest {
                    password: self.password.clone(),
                    token: self.token.clone(),
                };

                wasm_bindgen_futures::spawn_local(async move {
                    let auth = AuthContext::reset(req).await;
                    link.send_message_batch(vec![ResetMsg::ResetFinished(auth)]);
                });
                false
            }
            ResetMsg::ResetFinished(res) => match res {
                Ok(result) => {
                    self.response = result;
                    true
                }
                Err(err) => {
                    self.response = err;
                    true
                }
            },
            ResetMsg::PasswordChanged(v) => {
                self.password = v;
                true
            }
            ResetMsg::PasswordRepeatChanged(v) => {
                self.password_repeat = v;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let password_match = !self.password.is_empty() && self.password == self.password_repeat;
        let submit_disabled = !password_match;
        html! {
            <>
                <div class="reset-form-container">
                    <form class="reset-form">
                        <div class="reset-input-box">
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
                                class="reset-input"
                                type="password"
                                placeholder="password"
                                autocomplete="on"
                                value={self.password.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    ResetMsg::PasswordChanged(input.value())
                                })}
                            />
                        </div>
                        <div class="reset-input-box">
                            <div class="password-icon">
                                <Icon
                                        data={IconData::BOOTSTRAP_KEY}
                                        width={"2em".to_owned()}
                                        height={"2em".to_owned()}
                                />
                            </div>
                            <input
                                required=true
                                class="reset-input"
                                type="password"
                                placeholder="repeat password"
                                autocomplete="on"
                                value={self.password_repeat.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    ResetMsg::PasswordRepeatChanged(input.value())
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
                            class="reset-button"
                            type="button"
                            disabled={submit_disabled}
                            onclick={ctx.link().callback(|_| ResetMsg::Submit)}
                        >
                            {"reset"}
                        </button>
                    </form>
                </div>
            </>
        }
    }
}
