use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

use crate::native::NativeCounter;
use crate::react::ReactCounter;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    react_count: usize,
    native_count: usize,
}

pub enum Msg {
    ReactCounterChange(usize),
    NativeCounterChange(usize),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            react_count: 0,
            native_count: 0,
        };
        App { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ReactCounterChange(react_count) => {
                self.state.react_count = react_count;
            }
            Msg::NativeCounterChange(native_count) => {
                self.state.native_count = native_count;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        info!("rendered!");

        let on_native_counter_change = self
            .link
            .callback(|native_count| Msg::NativeCounterChange(native_count));
        let on_react_counter_change = self
            .link
            .callback(|react_count| Msg::ReactCounterChange(react_count));
        html! {
            <div>
              <NativeCounter
                react_counter={self.state.react_count}
                on_native_counter_change=on_native_counter_change
              />
              <ReactCounter
                native_counter={self.state.native_count}
                on_react_counter_change=on_react_counter_change
              />
            </div>
        }
    }
}
