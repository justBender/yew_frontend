use yew::{Children, Component, Context, Html, Properties, html};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Wrapper;

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct WrapperProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Wrapper {
    type Message = ();
    type Properties = WrapperProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="wrapper">
                   { for ctx.props().children.iter() }
                </div>
            </>
        }
    }
}
