use gloo_timers::callback::Interval;
use yew::prelude::*;

use super::FactContext;

#[derive(Properties, PartialEq, Debug)]
pub struct CounterProps {
    pub target: u64,
    pub length: usize,
    pub interval: u32,
    #[prop_or_default]
    pub suffix: String
}

pub enum CounterMessage {
    UpdateN, Uncover
}

pub struct Counter {
    n: u64,
    clock_handle: Option<Interval>
}
impl Component for Counter {
    type Message = CounterMessage;

    type Properties = CounterProps;

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CounterMessage::UpdateN => {
                self.n += 1;
                if self.n >= ctx.props().target {
                    self.clock_handle.take().map(Interval::cancel);
                }
                true
            }
            CounterMessage::Uncover => {
                let (context, _) = ctx.link().context::<FactContext>(Callback::noop()).unwrap();
                if !context.uncovered {
                    let link = ctx.link().clone();
                    self.clock_handle = Some(Interval::new(ctx.props().interval, move || {
                        link.send_message(CounterMessage::UpdateN);
                    }));
                    context.dispatch(true);
                    true
                } else {false}
            }
        }
    }

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            n: 0,
            clock_handle: None
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = format!("{:0width$}", self.n, width=ctx.props().length);
        let hover = ctx.link().callback(|_| CounterMessage::Uncover);
        let (context, _) = ctx.link().context::<FactContext>(Callback::noop()).unwrap();
        let hide = if context.uncovered {"show-counter"} else {"hide-counter"};
        html! {
            <span class={classes!("bold", hide)} onmouseenter={hover}>
                {text}{ctx.props().suffix.clone()}
            </span>
        }
    }
}