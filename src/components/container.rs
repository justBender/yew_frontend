use crate::components::context::auth_context::AuthContext;
use crate::components::side_bar::SideBar;
use crate::components::top_bar::TopBar;
use yew::{Callback, Children, Component, Context, ContextHandle, Html, Properties, html};

#[warn(unused_variables)]
pub struct Container {
    pub sidebar_active: bool,
    pub context: AuthContext,
    pub _listener: ContextHandle<AuthContext>,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,
    pub context_callback: Callback<AuthContext>,
}

pub enum ContainerMsg {
    ToggleMenu,
    UserChanged(AuthContext),
}

impl Component for Container {
    type Message = ContainerMsg;
    type Properties = ContainerProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (context, listener) = ctx
            .link()
            .context::<AuthContext>(ctx.link().callback(Self::Message::UserChanged))
            .expect("No UserContext found");
        Self {
            sidebar_active: false,
            context,
            _listener: listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: ContainerMsg) -> bool {
        match msg {
            ContainerMsg::ToggleMenu => {
                self.sidebar_active = !self.sidebar_active;
                true
            }

            ContainerMsg::UserChanged(user_context) => {
                self.context = user_context;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle = ctx.link().callback(move |_| ContainerMsg::ToggleMenu);
        html! {
            <>
                <div id="container">
                    <SideBar context_callback={ctx.props().context_callback.clone()} active={self.sidebar_active} />
                    <TopBar context_callback={ctx.props().context_callback.clone()} toggle={toggle} />
                    { for ctx.props().children.iter() }
                </div>
            </>
        }
    }
}