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
            <div class={classes!("main-flex")}>
                <header::Header />
                <fact::Fact direction={fact::FactDirection::Left}>
                    <span>
                        {"The average person throws away "}
                        <fact::counter::Counter target={400} suffix={"kg"} length={3} interval={5}/>
                        {" of waste every year."}
                    </span>
                    <fact::pictogram::Pictogram delay={15} row_width={10} total={57}>
                    <i class="gg-shopping-bag"></i>
                    </fact::pictogram::Pictogram>
                </fact::Fact>
                < MapComponent />
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
