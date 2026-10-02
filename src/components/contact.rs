use crate::components::context::auth_context::AuthContext;
use yew::{Children, Component, Context, ContextHandle, Html, Properties, html};

#[warn(unused_variables)]
pub struct Contact {
    pub auth_context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct ContactProps {
    #[prop_or_default]
    pub children: Children,
}

pub enum ContactMsg {
    AuthUpdate(AuthContext),
}

impl Component for Contact {
    type Message = ContactMsg;
    type Properties = ContactProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, handle) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(ContactMsg::AuthUpdate))
            .expect("auth update callback failed");
        Self {
            auth_context,
            _listener: handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match &msg {
            ContactMsg::AuthUpdate(auth_context) => {
                self.auth_context = auth_context.clone();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="contact-container">
                   <div class="contact-email">
                        <a href="mailto:bender@justbender.com" class="contact-link" target="_blank">
                            {"bender@justbender.com"}
                        </a>
                   </div>
                    <div class="contact-disclaimer">
                        {"Disclaimer"}
                   </div>
                   { for ctx.props().children.iter() }
                </div>
            </>
        }
    }
}
