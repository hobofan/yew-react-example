use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;

use crate::native::NativeCounter;
use crate::react::ReactCounter;

pub struct App {
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

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let state = State {
            react_count: 0,
            native_count: 0,
        };
        App { state }
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
}

impl Renderable<App> for App {
    fn view(&self) -> Html<Self> {
        info!("rendered!");
        html! {
            <div>
              <NativeCounter
                react_counter={self.state.react_count}
                on_native_counter_change=|native_count| {Msg::NativeCounterChange(native_count)}
              />
              <ReactCounter
                native_counter={self.state.native_count}
                on_react_counter_change=|react_count| {Msg::ReactCounterChange(react_count)}
              />
            </div>
        }
    }
}
