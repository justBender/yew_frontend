use yew::Html;
use crate::components::icons::svg_icon_props::SvgIconProps;

pub trait IconRenderer {
    fn svg(props: &SvgIconProps) -> Html;
}