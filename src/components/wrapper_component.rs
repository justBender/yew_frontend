use yew::{Component, Context, Html, html};
use crate::components::content::ContentComponent;

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct WrapperComponent {
    pub active: bool,
}

impl Component for WrapperComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: true }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="wrapper">
                    <ContentComponent active={true} image={"static/assets/BenderAndFerris_cutout.png"} />
                </div>
            </>
        }
    }
}
