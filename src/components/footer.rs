use yew::{Component, Context, Html, html};
use yew_icons::{Icon, IconData};

use crate::components::stack::Stack;

pub struct Footer;

pub struct FooterProperties;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="footer-container">
                    <div class="footer">
                        <div class="footer-text">
                            <div>{"made with"}</div>
                            <div class="footer-heart-container">
                                <Icon
                                    data={IconData::BOOTSTRAP_SUIT_HEART_FILL}
                                    width={"0.60rem".to_owned()}
                                    height={"0.60rem".to_owned()}
                                    class={"heart-icon"}
                                />
                            </div>
                            <div>{"and"}</div>
                        </div>
                        <Stack />
                    </div>
                </div>
            </>
        }
    }
}
