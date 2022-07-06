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
pub struct Uncovered {
    uncovered: bool
}
impl Reducible for Uncovered {
    type Action = bool;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self {uncovered: action}.into()
    }
}

type FactContext = UseReducerHandle<Uncovered>;

#[function_component(Fact)]
pub fn fact(props: &FactProps) -> Html {
    let mut chiter = props.children.iter();
    let text = chiter.next().unwrap_or_default();
    let graphic = chiter.next().unwrap_or_default();
    let context = use_reducer(|| Uncovered {uncovered: false});
    html! {
        <div class={classes!("fact")}>
            <ContextProvider<FactContext> {context}>
            <div class={classes!("fact-info", props.direction.class())}>{text}</div>
            <div class={classes!("fact-graphic")}>{graphic}</div>
            </ContextProvider<FactContext>>
        </div>
    }
}