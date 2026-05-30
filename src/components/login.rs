use yew::{html, Component, Context, Html, Properties};
use yew_icons::{Icon, IconData};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct LoginComponent {
    username:String,
    password:String,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct LoginComponentProps {
}

impl Component for LoginComponent {
    type Message = ();
    type Properties = LoginComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="login-form-container">
                    <form class="login-form">
                        <div class="email">
                            <div class="email-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_PERSON_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                                />
                            </div>
                            <input class="email-input" autocomplete="email" name="email" placeholder="email" type="email"/>
                        </div>
                        <div class="password">
                            <div class="password-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_KEY_FILL}
                                    width={"2em".to_owned()}
                                    height={"2em".to_owned()}
                                />
                            </div>
                            <input class="password-input" autocomplete="password" name="password" placeholder="password" type="password"/>
                        </div>
                        <div class="reglog">
                            <button type="submit" class="submit-button">
                                {"login"}
                            </button>
                            <div class="pass-reset-link">
                                <a href="pass-reset" class="pass-reset">{"password-reset"}</a>
                            </div>
                            <div class="register-link">
                                <a href="register">{"or register?"}</a>
                            </div>
                        </div>
                    </form>
                </div>
            </>
        }
    }
}