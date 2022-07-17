mod map;
mod header;
mod fact;
mod messages;
use yew::prelude::*;
use map::MapComponent;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound
}

#[function_component]
pub fn App() -> Html{
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

fn main() {
    yew::Renderer::<App>::new().render();
}
