use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Default)]
pub struct RegisterComponent {
    username: String,
    email: String,
    password: String,
    password_repeat: String,
}

pub enum Msg {
    UpdateUsername(String),
    UpdateEmail(String),
    UpdatePassword(String),
    UpdatePasswordRepeat(String),
    Submit,
}

impl Component for RegisterComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(v) => {
                self.username = v;
                true
            }
            Msg::UpdateEmail(v) => {
                self.email = v;
                true
            }

            Msg::UpdatePassword(v) => {
                self.password = v;
                true
            }

            Msg::UpdatePasswordRepeat(v) => {
                self.password_repeat = v;
                true
            }
            Msg::Submit => {
                // send to backend later
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let password_match =
        !self.password.is_empty() && self.password == self.password_repeat;
        let submit_disabled = !password_match;
        html! {
            <div class="register-form-container">
                <form class="register-form">
                    <div class="register-input-box">
                       <input
                            required=true
                            class="register-input"
                            type="text"
                            placeholder="username"
                            value={self.username.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateUsername(input.value())
                            })}
                        />
                    </div>
                    <div class="register-input-box">
                        <input
                            required=true
                            class="register-input"
                            type="email"
                            placeholder="email"
                            value={self.email.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdateEmail(input.value())
                            })}
                        />
                    </div>
                    <div class="register-input-box">
                        <input
                            required=true
                            class="register-input"
                            type="password"
                            placeholder="password"
                            value={self.password.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdatePassword(input.value())
                            })}
                        />
                    </div>
                    <div class="register-input-box">
                        <input
                            required=true
                            class="register-input"
                            type="password"
                            placeholder="repeat password"
                            value={self.password_repeat.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::UpdatePasswordRepeat(input.value())
                            })}
                        />
                    </div>
                    {
                        if password_match {
                            html! {
                                <p class="passmatch">
                                    {"passwords match"}
                                </p>
                            }
                        } else {
                            html! {
                                <p class="passnomatch">
                                    {"passwords must match"}
                                </p>
                            }
                        }
                    }
                    <button
                        class="register-button"
                        type="submit"
                        disabled={submit_disabled}
                        onclick={ctx.link().callback(|_| Msg::Submit)}
                    >
                        {"Register"}
                    </button>
                </form>
            </div>
        }
    }
}