use wasm_bindgen::prelude::*;
use web_sys::{Element, Node};
use yew::virtual_dom::VNode;
use yew::{html, Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct ReactButton {
    props: ReactButtonProps,
}

#[derive(Properties, Clone)]
pub struct ReactButtonProps {
    pub text: String,
}

impl Component for ReactButton {
    type Message = ();
    type Properties = ReactButtonProps;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        log::debug!("CREATE");
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!("UPDATE");
        // match msg {
        // Self::Message::NewProps(new_props) => self.change(new_props),
        // }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!("CHANGE");
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <button>
                {&self.props.text}
            </button>
        }
    }
}

////////////
////////////
////////////

// pub enum Msg {
// NewProps(ReactButtonProps),
// }

// #[wasm_bindgen(module = "/src/react.js")]
// extern "C" {
// #[wasm_bindgen(js_name = "render_material_ui_chip")]
// fn render_material_ui_chip_js(node: &Node, label: String, on_click: JsValue);
// }

#[wasm_bindgen]
pub fn render_react_button(element: Element, text: String) -> JsValue {
    yew::initialize();

    let link =
        yew::app::App::<ReactButton>::new().mount_with_props(element, ReactButtonProps { text });
    yew::run_loop();

    Closure::wrap(Box::new(move |element, newText| {
        log::debug!("Trying to trigger update!");
        log::debug!("Update: {:?}", element);
        log::debug!("Update: {:?}", newText);
        // let new_props = ReactButtonProps { text: newText };
        // link.send_message(Msg::NewProps(new_props));
    }) as Box<dyn Fn(Element, String)>)
    .into_js_value()
}
