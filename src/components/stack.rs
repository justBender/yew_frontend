use crate::components::icons::{
    actix::ActixIcon, caddy::CaddyIcon, icon::IconRenderer, svg_icon_props::SvgIconProps,
    yew::YewIcon,
};
use yew::{Children, Component, Context, Html, Properties, classes, html};
use yew_icons::{Icon, IconData};

pub struct Stack;

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct StackProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Stack {
    type Message = ();
    type Properties = StackProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <>
            <div class="stack">
                <div class="stack-container">
                    <div class="stack-element">
                        <a class="stack-link" href="https://rust-lang.org" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                    <Icon
                                        data={IconData::SIMPLE_ICONS_RUST}
                                        width={"0.75rem".to_owned()}
                                        height={"0.75rem".to_owned()}
                                        class={"rust-icon"}
                                    />
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://actix.rs" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                    {
                                        ActixIcon::svg(
                                            &SvgIconProps {
                                                class: classes!["stack-icon"],
                                                width: 12,
                                                height: 12,
                                                style: Default::default(),
                                                view_box: [0.0, 0.0, 24.0, 24.0]
                                            }
                                        )
                                    }
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://yew.rs" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">{
                                YewIcon::svg(
                                    &SvgIconProps {
                                        class: classes!["stack-icon"],
                                        width: 12,
                                        height: 12,
                                        style: Default::default(),
                                        view_box: [7.0, 10.0, 24.0, 62.0]
                                    }
                                )
                            }
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://tailwindcss.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_TAILWINDCSS}
                                    width={"0.75rem".to_owned()}
                                    height={"0.75rem".to_owned()}
                                    class={"tailwindcss-icon"}
                                    title={"tailwindcss.com"}
                                />
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://neo4j.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_NEO_4_J}
                                    width={"0.75rem".to_owned()}
                                    height={"0.75rem".to_owned()}
                                    class={"stack-icon"}
                                />
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://docker.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_DOCKER}
                                    width={"0.75rem".to_owned()}
                                    height={"0.75rem".to_owned()}
                                    class={"docker-icon"}
                                />
                            </div>
                       </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://pnpm.io" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_PNPM}
                                    width={"0.75rem".to_owned()}
                                    height={"0.75rem".to_owned()}
                                    class={"stack-icon"}
                                />
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://caddyserver.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">{
                                CaddyIcon::svg(
                                    &SvgIconProps {
                                        class: classes!["stack-icon"],
                                        width: 12,
                                        height: 12,
                                        style: Default::default(),
                                        view_box: [0.0, 0.0, 156.78, 147.0]
                                    }
                                )
                            }
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://git-scm.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_GIT}
                                    width={"0.75rem".to_owned()}
                                    height={"0.75rem".to_owned()}
                                    class={"git-icon"}
                                />
                            </div>
                        </a>
                    </div>
                </div>
                { for ctx.props().children.iter() }
            </div>
        </>
        }
    }
}
