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
                    <fact::pictogram::Pictogram delay={5} row_width={10} total={57}>{"h"}</fact::pictogram::Pictogram>
                </fact::Fact>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
