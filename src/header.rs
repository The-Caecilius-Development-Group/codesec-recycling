use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Header
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("header")}>
                <div class={classes!("divider")}></div>
                <div class={classes!("recycling-header")}>{"Recycling"}</div>
                <div class={classes!("header-question")}>{"What's it worth?"}</div>
            </div>
        }
    }
}