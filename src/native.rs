use yew::prelude::*;

pub struct NativeCounter {
    link: ComponentLink<Self>,
    props: NativeCounterProps,
    native_counter: usize,
}

#[derive(Properties, Clone)]
pub struct NativeCounterProps {
    #[props(required)]
    pub react_counter: usize,
    #[props(required)]
    pub on_native_counter_change: Callback<usize>,
}

pub enum Msg {
    Increment,
}

impl Component for NativeCounter {
    type Message = Msg;
    type Properties = NativeCounterProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NativeCounter {
            link,
            props,
            native_counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Increment => {
                self.native_counter += 1;
                self.props
                    .on_native_counter_change
                    .emit(self.native_counter);
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
            self.native_counter, self.props.react_counter
        );
        let onclick = self.link.callback(|_| Msg::Increment);
        html! {
            <button onclick=onclick>{label}</button>
        }
    }
}
