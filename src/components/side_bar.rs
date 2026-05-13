use crate::components::container_component::Msg;
use yew::{Component, Context, Html, Properties, html};
use yew_icons::{Icon, IconData};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct SideBar {
    pub active: bool,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct SideBarProps {
    pub active: bool,
}

impl Component for SideBar {
    type Message = Msg;
    type Properties = SideBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: false }
    }

    fn update(&mut self, __ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.active = !self.active;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        /*let visible = if ctx.props().active {
            "side-bar"
        } else {
            "side-bar invisible"
        };*/
        html! {
            <>
                <div class="side-bar">
                    <div class="side-bar-el">
                        <Icon
                            data={IconData::BOOTSTRAP_GIT}
                            width={"1.5rem".to_owned()}
                            height={"1.5rem".to_owned()}
                            class={"side-bar-icon"}
                        />
                       <div class="side-bar-el-label">{"git"}</div>
                    </div>
                    <div class="side-bar-el">
                        <Icon
                            data={IconData::BOOTSTRAP_ENVELOPE_AT}
                            width={"1.5rem".to_owned()}
                            height={"1.5rem".to_owned()}
                            class={"side-bar-icon"}
                        />
                       <div class="side-bar-el-label">{"contact"}</div>
                    </div>
                    <div class="side-bar-el">
                        <Icon
                            data={IconData::BOOTSTRAP_BRACES_ASTERISK}
                            width={"1.5rem".to_owned()}
                            height={"1.5rem".to_owned()}
                            class={"side-bar-icon"}
                        />
                        <div class="side-bar-el-label">{"stack"}</div>
                    </div>
                    <div class="side-bar-el">
                        <Icon
                            data={IconData::BOOTSTRAP_QUOTE}
                            width={"1.5rem".to_owned()}
                            height={"1.5rem".to_owned()}
                            class={"side-bar-icon"}
                        />
                        <div class="side-bar-el-label">{"blog"}</div>
                    </div>
                </div>
            </>
        }
    }
}
