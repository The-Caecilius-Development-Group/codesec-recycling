use leaflet::{LatLng, Map, TileLayer, Marker};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::{
    Html, Component
};
use web_sys::{Element, HtmlElement, Node}; 
use gloo_utils::document;

pub enum Msg {
    Print
}

pub struct MapComponent {
    map: Map,
    container: HtmlElement,
    n: u64, 

}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
}

impl MapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for MapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &JsValue::NULL);
        Self {
            map: leaflet_map,
            container,
            n: 0
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // Makes the map come into view if it is the first time the map is being shown after reloading.
        if first_render {
            self.map.setView(&LatLng::new(51.509865, -0.118092), 11.0);
            add_tile_layer(&self.map);
            let marker = Marker::new(&LatLng::new(51.63544609737626, -0.4685434773325837));
            marker.addTo(&self.map);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Print => {
                self.n += 1;
            }
        } true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let text = format!("{}", self.n);
        let print = ctx.link().callback(|_| Msg::Print);
        html! {
            <div class="map-container component-container">
                {self.render_map()}
                <button onclick={print}>{text}</button>
            </div>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new(
        "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
        &JsValue::NULL,
    )
    .addTo(map);
}
