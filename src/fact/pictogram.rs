use gloo_timers::callback::Interval;
use yew::{*, context::ContextHandle};

use super::{Uncovered, FactContext};

#[derive(Properties, PartialEq, Debug)]
pub struct PictogramProps {
    pub row_width: usize,
    pub total: usize,
    pub children: Children,
    pub delay: u32
}

pub enum PictogramMessage {
    UpdateGraph, ContextUpdate(FactContext)
}

pub struct Pictogram {
    clock_handle: Option<Interval>,
    _context_handle: ContextHandle<FactContext>,
    rows: Vec<Vec<Html>>,
    element: Html,
    n: usize
}
impl Component for Pictogram {
    type Message = PictogramMessage;

    type Properties = PictogramProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (_, _context_handle) = ctx.link()
            .context(ctx.link().callback(|context| {
                log::info!("callback context update");
                PictogramMessage::ContextUpdate(context)
            }))
            .unwrap();
        log::info!("yeah I've got a context {:?}", _context_handle);
        Pictogram { 
            clock_handle: None,
            rows: vec![vec![]],
            element: ctx.props().children.iter().next().unwrap(),
            n: 0,
            _context_handle
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("update");
        match msg {
            PictogramMessage::UpdateGraph => {
                
                if self.rows.last().unwrap().len() == ctx.props().row_width {
                    // Make a new row
                    self.rows.push(vec![self.element.clone()]);
                } else {
                    self.rows.last_mut().unwrap().push(self.element.clone());
                }
                self.n += 1;
                if self.n >= ctx.props().total {self.clock_handle.take().map(Interval::cancel);}
                true
            }
            PictogramMessage::ContextUpdate(context) => {
                if self.clock_handle.is_none() && context.uncovered {
                    let link = ctx.link().clone();
                    self.clock_handle = Some(
                        Interval::new(ctx.props().delay, move || link.send_message(PictogramMessage::UpdateGraph))
                    );
                }
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let rendered_rows = self.rows.iter().map(
            |r| html!(<div class="pictogram-row">{r.clone()}</div>)
        );
        html! {
            <div class="pictogram">
                {for rendered_rows}
            </div>
        }
    }
}