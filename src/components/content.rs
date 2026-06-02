use yew::{Component, Context, Html, Properties, html, Children};

#[warn(unused_variables)]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct ContentComponent {
    pub active: bool,
}

#[warn(unused_variables)]
#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct ContentComponentProps {
    pub active: bool,
    #[prop_or_default]
    pub children: Children,
    pub image: Option<String>,
}

impl Component for ContentComponent {
    type Message = ();
    type Properties = ContentComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool { false }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="content">
                    <article class="content-article">
                        <div class="content-article-headline">
                            {"Rust Evangelists finally got me."}
                        </div>
                        <img class="content-image" alt="logo" src={ctx.props().image.clone().unwrap_or_default()}/>
                        <div class="content-article-body">
                            {"Content"}
                        </div>
                    </article>
                </div>
            </>
        }
    }
}
