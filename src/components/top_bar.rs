use crate::components::container_component::Msg;
use yew::{Callback, Component, Context, Html, Properties, html};
use yew_icons::{Icon, IconData};
use yew_router::prelude::Link;
use crate::components::routing::router::Route;

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct TopBar {
    pub active: bool,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct TopBarProps {
    pub toggle: Callback<()>,
}

impl Component for TopBar {
    type Message = Msg;
    type Properties = TopBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: true }
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
        let call_back = ctx.props().toggle.clone();
        html! {
            <>
                <div class="top-bar">
                    <div class="bar-el">
                        <button class="burger-button" onclick={call_back.reform(move|_| ())}>
                            <Icon
                                data={IconData::HEROICONS_SOLID_BARS_3}
                                width={"1.5em".to_owned()}
                                height={"1.5em".to_owned()}
                            />
                        </button>
                    </div>
                    <div class="bar-menu">
                        <div class="menu-el">
                            <Link<Route> to={Route::Home}>
                                <Icon
                                    data={IconData::HEROICONS_SOLID_HOME}
                                    width={"1.25em".to_owned()}
                                    height={"1.25em".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                        <div class="menu-el">
                                <Link<Route> to={Route::LandingPage}>
                                    <Icon
                                            data={IconData::OCTICONS_GEAR_24}
                                            width={"1.25em".to_owned()}
                                            height={"1.25em".to_owned()}
                                    />
                                </Link<Route>>
                        </div>
                        <div class="menu-el">
                            <Link<Route> to={Route::LoginPage}>
                                <Icon
                                    data={IconData::OCTICONS_SIGN_IN_24}
                                    width={"1.25em".to_owned()}
                                    height={"1.25em".to_owned()}
                                />
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
