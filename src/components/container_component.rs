use crate::components::side_bar::SideBar;
use crate::components::top_bar::TopBar;
use crate::components::wrapper_component::WrapperComponent;
use yew::{Component, Context, Html, Properties, html, Children};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ContainerComponent {
    pub active: bool,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct ContainerComponentProps {
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
}

pub enum Msg {
    ToggleMenu,
}

impl Component for ContainerComponent {
    type Message = Msg;
    type Properties = ContainerComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.active = !self.active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggled = ctx.link().callback(|_| Msg::ToggleMenu);
        html! {
            <>
                <div id="container">
                    <SideBar active={self.active} />
                    <TopBar toggle={toggled} />
                    <WrapperComponent />
                    { for ctx.props().children.iter() }
                </div>
            </>
        }
    }
}
