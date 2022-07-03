use yew::prelude::*;

mod header;
mod fact;

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
                <fact::Fact direction={fact::FactDirection::Left}>
                    <span>
                        {"The average person throws away "}
                        <fact::counter::Counter target={400} suffix={"kg"} length={3} interval={5}/>
                        {" of waste every year."}
                    </span>
                </fact::Fact>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
