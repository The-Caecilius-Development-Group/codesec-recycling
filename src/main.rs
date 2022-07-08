mod map;
mod header;
mod fact;
mod messages;
use yew::prelude::*;
use map::MapComponent;

enum Msg {
}

struct Model {
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    } 

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
                <messages::Messages />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
