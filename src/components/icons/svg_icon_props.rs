use yew::{AttrValue, Classes, Properties};

#[derive(Properties, Clone, PartialEq, Debug, Default)]
pub struct SvgIconProps {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default(24)]
    pub width: u32,

    #[prop_or_default(24)]
    pub height: u32,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default([0.0, 0.0, 24.0, 24.0])]
    pub view_box: [f32;4],
}