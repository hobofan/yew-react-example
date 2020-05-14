use std::convert::TryInto;
use stdweb::js;
use stdweb::web::Node;
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
            node: stdweb::web::document()
                .create_element("div")
                .unwrap()
                .try_into()
                .unwrap(),
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
        // Wrap callback in a closure that we can use in the js! macro
        let orig_callback = self.react_counter_cb.clone();
        let callback = move || orig_callback.emit(());

        let label = format!(
            "Native count: {} - React count: {}",
            self.props.native_counter, self.react_counter
        );
        js! {
            let element = React.createElement(MaterialUI.Chip,
                {
                  label: @{label},
                  onClick: () => @{callback}(),
                }
              );
            ReactDOM.render(element, @{self.node.clone()});
        }

        yew::virtual_dom::VNode::VRef(self.node.clone())
    }
}

impl ReactCounter {
    fn link_react_counter_cb(link: &mut ComponentLink<Self>) -> Callback<()> {
        link.callback(|_| Msg::Increment)
    }
}
