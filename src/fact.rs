use yew::prelude::*;

pub mod counter;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum FactDirection {Left, Right}
impl FactDirection {
    fn class(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right"
        }
    }
}

#[derive(PartialEq, Clone, Debug, Properties)]
pub struct FactProps {
    pub direction: FactDirection,
    pub children: Children
}

pub struct Fact;
impl Component for Fact {
    type Message = ();

    type Properties = FactProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Fact
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = ctx.props().children.iter().next().unwrap_or_default();
        html! {
            <div class={classes!("fact")}>
                <div class={classes!("fact-info", ctx.props().direction.class())}>{text}</div>
            </div>
        }
    }
}