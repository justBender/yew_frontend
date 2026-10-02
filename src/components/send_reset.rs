use crate::components::context::auth_context::AuthContext;
use crate::services::send_reset::SendResetRequest;
use web_sys::{HtmlInputElement, InputEvent};
use yew::{Component, Context, Html, Properties, TargetCast, html};
use yew_icons::{Icon, IconData};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SendReset {
    pub email: String,
    pub response: String,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct SendResetProps;

pub enum SendResetMsg {
    EmailChanged(String),
    Submit,
    SubmitFinished(Result<String, String>),
}

impl Component for SendReset {
    type Message = SendResetMsg;
    type Properties = SendResetProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            email: "".to_string(),
            response: "".to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        match msg {
            SendResetMsg::Submit => {
                let reset_request = SendResetRequest {
                    email: self.email.clone(),
                };
                wasm_bindgen_futures::spawn_local(async move {
                    link.send_message(SendResetMsg::SubmitFinished(
                        AuthContext::send_reset(reset_request).await,
                    ));
                });
                true
            }
            SendResetMsg::SubmitFinished(res) => match res {
                Ok(result) => {
                    self.response = result.clone();
                    true
                }
                Err(e) => {
                    self.response = e.to_string();
                    true
                }
            },
            SendResetMsg::EmailChanged(email) => {
                self.email = email;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="send-reset-form-container">
                    <form class="send-reset-form">
                        <div class="send-reset-hint">
                            {
                                if self.response.is_empty() {
                                    "enter your email address \n and press 'send reset'".to_string()
                                } else {
                                    self.response.clone()
                                }
                            }
                        </div>
                        <div class="send-reset-email">
                            <div class="send-reset-email-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_PERSON_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                                />
                            </div>
                            <input
                                required=true
                                name="email"
                                class="send-reset-input"
                                type="text"
                                placeholder="email for the reset"
                                autocomplete="on"
                                value={self.email.clone()}
                                oninput={ctx.link().callback(|e: InputEvent| {
                                    let input: HtmlInputElement = e.target_unchecked_into();
                                    SendResetMsg::EmailChanged(input.value())
                                })}
                            />
                        </div>
                        <div class="send-reset-button-box">
                            <button type="button" class="send-reset-submit-button" onclick={ctx.link().callback(|_| SendResetMsg::Submit )}>
                                {"send reset"}
                            </button>
                        </div>
                    </form>
                </div>
            </>
        }
    }
}
