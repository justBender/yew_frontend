use crate::components::icons::icon::IconRenderer;
use crate::components::icons::svg_icon_props::SvgIconProps;
use yew::{Component, Context, Html, html};

#[derive(Clone, Debug, Default)]
pub struct YewIcon;

impl Component for YewIcon {
    type Message = ();
    type Properties = SvgIconProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        Self::svg(ctx.props())
    }
}

impl IconRenderer for YewIcon {
    fn svg(props: &SvgIconProps) -> Html {
        let width = props.width.to_string();
        let height = props.height.to_string();
        let view_box: String = format!(
            "{} {} {} {}",
            props.view_box[0], props.view_box[1], props.view_box[2], props.view_box[3]
        );
        html! {
            <>
                <svg width={width} height={height} viewBox={view_box} preserveAspectRatio="xMinYMin meet" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <circle cx="38" cy="40" r="25" fill="#B3E1CE" stroke="#FFFFFF" stroke-width="4"/>
                    <path d="M38.2373 41.0339L14 14" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
                    <path d="M38.2373 41.0339L62.4746 14" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
                    <path d="M38.2373 41.0339L38.2373 69" stroke="#444444" stroke-width="6" stroke-linecap="round"/>
                    <circle cx="38" cy="41" r="7" fill="#009A5B" stroke="#444444" stroke-width="4"/>
                </svg>
            </>
        }
    }
}

