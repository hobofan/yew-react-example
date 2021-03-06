use wasm_bindgen::prelude::*;
use web_sys::Node;
use yew::virtual_dom::VNode;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender};

pub struct ReactCounter {
    node: Node,
    props: ReactCounterProps,
    react_counter_cb: Callback<()>,
    react_counter: usize,
}

#[derive(Properties, Clone)]
pub struct ReactCounterProps {
    pub native_counter: usize,
    pub on_react_counter_change: Callback<usize>,
}

pub enum Msg {
    Increment,
}

impl Component for ReactCounter {
    type Message = Msg;
    type Properties = ReactCounterProps;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        ReactCounter {
            // Creating an element that we can render the React component into later
            node: Node::from(
                web_sys::window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element("div")
                    .unwrap(),
            ),
            props,
            // Creating a wrapper for the counter callback
            react_counter_cb: Self::link_react_counter_cb(&mut link),
            react_counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                // Increment internal counter
                self.react_counter += 1;
                // Invoke callback with new count
                self.props.on_react_counter_change.emit(self.react_counter);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let label = format!(
            "Native count: {} - React count: {}",
            self.props.native_counter, self.react_counter
        );
        render_material_ui_chip(&self.node, label, self.react_counter_cb.clone());

        VNode::VRef(self.node.clone())
    }
}

#[wasm_bindgen(module = "/src/react.js")]
extern "C" {
    #[wasm_bindgen(js_name = "render_material_ui_chip")]
    fn render_material_ui_chip_js(node: &Node, label: String, on_click: JsValue);
}

fn render_material_ui_chip(node: &Node, label: String, on_click: Callback<()>) {
    let callback = Closure::once_into_js(move || on_click.emit(()));
    render_material_ui_chip_js(node, label, callback)
}

impl ReactCounter {
    fn link_react_counter_cb(link: &mut ComponentLink<Self>) -> Callback<()> {
        link.callback(|_| Msg::Increment)
    }
}
