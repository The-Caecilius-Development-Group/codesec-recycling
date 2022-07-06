use std::{cell::RefCell, rc::Rc};

use yew::prelude::*;

pub mod counter;
pub mod pictogram;

#[derive(PartialEq, Clone, Copy, Debug)]
#[allow(dead_code)]
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

#[derive(Clone, PartialEq, Debug)]
pub struct FactContext {
    uncovered: bool
}
impl FactContext {
    pub fn get<T: Component>(ctx: &Context<T>) -> Self {
        let (context, _) = ctx.link().context::<FactContext>(Callback::noop()).unwrap();
        context
    }
}

#[function_component]
pub fn Fact(props: &FactProps) -> Html {
    let mut chiter = props.children.iter();
    let text = chiter.next().unwrap_or_default();
    let graphic = chiter.next().unwrap_or_default();
    let context = FactContext(Rc::new(RefCell::new(false)));
    html! {
        <div class={classes!("fact")}>
            <ContextProvider<FactContext> {context}>
            <div class={classes!("fact-info", props.direction.class())}>{text}</div>
            <div class={classes!("fact-graphic")}>{graphic}</div>
            </ContextProvider<FactContext>>
        </div>
    }
}