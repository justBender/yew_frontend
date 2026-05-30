use yew::{html, Component, Context, Html, Properties};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct RegisterComponent {
    username:String,
    password:String,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct RegisterComponentProps {
}

impl Component for RegisterComponent {
    type Message = ();
    type Properties = RegisterComponentProps;

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
                <div class="login-form">
                    <form>
                        <div class="email">
                            <input autocomplete="email" name="email" placeholder="email" type="text"/>
                        </div>
                        <div class="password">
                            <input class="password" name="password" placeholder="password" type="password"/>
                        </div>
                        <div class="submit-button">
                            {"Login"}
                        </div>
                    </form>
                </div>
            </>
        }
    }
}