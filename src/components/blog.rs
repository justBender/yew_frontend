use crate::components::context::auth_context::AuthContext;
use yew::{Children, Component, Context, ContextHandle, Html, Properties, html};

#[warn(unused_variables)]
pub struct Blog {
    pub auth_context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct BlogProps {
    #[prop_or_default]
    pub children: Children,
    pub image: Option<String>,
}

pub enum BlogMsg {
    AuthUpdate(AuthContext),
}

impl Component for Blog {
    type Message = BlogMsg;
    type Properties = BlogProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (auth_context, handle) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(BlogMsg::AuthUpdate))
            .unwrap();
        Self {
            auth_context,
            _listener: handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="content">
                    <article class="content-article">
                        <div class="content-article-headline">
                            {"Becoming rusty."}
                        </div>
                        <img class="content-image" alt="logo" src={ctx.props().image.clone().unwrap_or_default()}/>
                        <div class="content-article-body">
                            <p class="content-article-subtitle">
                                {"Aged guy getting even more rusty:"}
                            </p>
                            {"
                            felt like documenting some of my Rust(lang) 'journey'
                            and maybe somebody else can gain something from this beyond
                            me just archiving this.
                            "}
                        </div>
                    </article>
                </div>
            </>
        }
    }
}
