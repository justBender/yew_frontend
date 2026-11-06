use yew::{Children, Classes, Component, Context, Html, Properties, html};
use yew_icons::{Icon, IconData};

#[derive(Properties, PartialEq, Clone)]
pub struct StackElement {
    pub data: IconData,
    pub width: String,
    pub height: String,
    pub class: Classes,
    pub href: String,
}

#[warn(unused_variables)]
#[derive(Properties, PartialEq)]
pub struct StackElementProps {
    pub data: IconData,
    pub width: String,
    pub height: String,
    pub class: Classes,
    pub href: String,
    #[prop_or_default]
    pub children: Children,
}

impl Component for StackElement {
    type Message = ();
    type Properties = StackElementProps;

    fn create(ctx: &Context<Self>) -> StackElement {
        let props = ctx.props();
        StackElement {
            data: props.data,
            width: props.width.clone(),
            height: props.height.clone(),
            class: props.class.clone(),
            href: props.href.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let href = ctx.props().href.clone();
        html! {
            <>
                <div class="stack-element">
                    <a class="stack-link" href={ href } target="_blank" rel="noopener noreferrer" role="link">
                        <div class="stack-icon">
                           <Icon
                                data={self.data}
                                width={self.width.clone()}
                                height={self.height.clone()}
                                class={self.class.clone()}
                           />
                        </div>
                    </a>
                </div>
            </>
        }
    }
}
