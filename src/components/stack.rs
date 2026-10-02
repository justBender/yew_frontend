use crate::components::stack_element::StackElement;
use yew::{Children, Component, Context, Html, Properties, html};

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
        let icon_list: Vec<StackElement> = vec![StackElement {
            name: "BOOTSTRAP_SUIT_HEART_FILL".to_string(),
            icon_props: yew_icons::IconProps {
                data: yew_icons::IconData::BOOTSTRAP_SUIT_HEART_FILL,
                title: Some("Bootstrap Suit Heart Fill".to_string()),
                width: "1.0rem".to_string(),
                height: "1.0rem".to_string(),
                class: "heart-icon".to_string(),
            },
        }];
        html! {
        <>
            <div class="stack">
                { for icon_list.iter().map(move |item|
                    html! {
                        <div class="stack-element">
                            <StackElement
                                name={item.name}
                                icon_props={item.icon_props}
                                href={item.href}
                            />
                        </div>
                    })
                }
                /*
                <div class="stack-gist">
                    <div>
                        {r#"
                                this site and its
                                services have been built
                                with
                            "#}
                        <Icon
                            data={IconData::BOOTSTRAP_SUIT_HEART_FILL}
                            width={"1.0rem".to_owned()}
                            height={"1.0rem".to_owned()}
                            class={"heart-icon"}
                        />
                        {r#"
                                and:
                            "#}
                    </div>
                </div>
                <div class="stack-container">
                    <div class="stack-element">
                        <a class="stack-link" href="https://rust-lang.org" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                    <Icon
                                        data={IconData::SIMPLE_ICONS_RUST}
                                        width={"1.5rem".to_owned()}
                                        height={"1.5rem".to_owned()}
                                        class={"rust-icon"}
                                    />
                            </div>
                            <div class="stack-content">
                                {"rust"}
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
                                                width: 24,
                                                height: 24,
                                                style: Default::default(),
                                                view_box: [0.0, 0.0, 24.0, 24.0]
                                            }
                                        )
                                    }
                            </div>
                            <div class="stack-content">
                                    {"actix"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://yew.rs" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">{
                                YewIcon::svg(
                                    &SvgIconProps {
                                        class: classes!["stack-icon"],
                                        width: 24,
                                        height: 24,
                                        style: Default::default(),
                                        view_box: [7.0, 10.0, 24.0, 62.0]
                                    }
                                )
                            }
                            </div>
                            <div class="stack-content">
                                {"yew"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://tailwindcss.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_TAILWINDCSS}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                    class={"tailwindcss-icon"}
                                    title={"tailwindcss.com"}
                                />
                            </div>
                            <div class="stack-content">
                                {"tailwindcss"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://neo4j.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_NEO_4_J}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                    class={"stack-icon"}
                                />
                            </div>
                            <div class="stack-content">
                                {"neo4j + neo4rs"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://docker.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_DOCKER}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                    class={"docker-icon"}
                                />
                            </div>
                            <div class="stack-content">
                                {"docker"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://pnpm.io" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::SIMPLE_ICONS_PNPM}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                    class={"stack-icon"}
                                />
                            </div>
                            <div class="stack-content">
                                {"pnpm"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://caddyserver.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">{
                                CaddyIcon::svg(
                                    &SvgIconProps {
                                        class: classes!["stack-icon"],
                                        width: 24,
                                        height: 24,
                                        style: Default::default(),
                                        view_box: [0.0, 0.0, 156.77999999999997, 147.0]
                                    }
                                )
                            }
                            </div>
                            <div class="stack-content">
                                {"caddy"}
                            </div>
                        </a>
                    </div>
                    <div class="stack-element">
                        <a class="stack-link" href="https://git-scm.com" target="_blank" rel="noopener noreferrer" role="link">
                            <div class="stack-icon">
                                <Icon
                                    data={IconData::BOOTSTRAP_GIT}
                                    width={"1.5rem".to_owned()}
                                    height={"1.5rem".to_owned()}
                                    class={"git-icon"}
                                />
                            </div>
                            <div class="stack-content">
                                {"git"}
                            </div>
                        </a>
                    </div>
                </div>
                */
                { for ctx.props().children.iter() }
            </div>
        </>
        }
    }
}
