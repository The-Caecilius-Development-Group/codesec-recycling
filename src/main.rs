mod map;
mod header;
mod fact;
use yew::prelude::*;
use map::MapComponent;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    } 

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div class={classes!("main-flex")}>
                <header::Header />
                <fact::Fact direction={fact::FactDirection::Left}>
                    <span>
                        {"The average person throws away "}
                        <fact::counter::Counter target={400} suffix={"kg"} length={3} interval={5}/>
                        {" of waste every year."}
                    </span>
                </fact::Fact>
                <MapComponent / >
            </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
