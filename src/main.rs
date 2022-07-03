use yew::prelude::*;

mod header;

struct App;

impl Component for App {
    type Message = ();

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("main-flex")}>
                <header::Header />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
