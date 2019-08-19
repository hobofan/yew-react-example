## About

This template shows how to create a web app using a React component inside a Yew component.

## 🚴 Usage

### 🛠️ Build with `npm run build`

```
yarn run build
```

### 🔬 Serve locally with `yarn npm start:dev`

```
npm run start:dev
```

## 🔎 Explanation

### Including dependencies

In the `index.html`, we include `react` and `react-dom` as UMD packages (See [React docs](https://reactjs.org/docs/cdn-links.html)).

Additionally we include `material-ui` so that we have some components available the we can use in the example.

```html
  <script crossorigin src="./react.development.js"></script>
  <script crossorigin src="./react-dom.development.js"></script>
  <script crossorigin src="./material-ui.development.js"></script>
```

### Yew component that uses a React component

Inside [src/react.rs](./src/react.rs) you can find the Yew component `ReactCounter` that internally uses a React component to display a button with an incrmenting counter.

#### Constructor (fn create)

In the `create` function, we [create a new element](https://docs.rs/stdweb/0.4.18/stdweb/web/struct.Document.html#method.create_element), which we will later use to render the React component into:

```rust
fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    ReactCounter {
        // ...
        node: stdweb::web::document()
            .create_element("div")
            .unwrap()
            .try_into()
            .unwrap(),
        // ...
    }
}
```

We also create a [Callback](https://docs.rs/yew/0.8.0/yew/callback/struct.Callback.html) wrapper, which we need to create a Message for our Component from a JS callback:

```rust
fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
    ReactCounter {
        // ...
        react_counter_cb: Self::link_react_counter_cb(&mut link),
        // ...
    }
}
```

#### Rendering the component (fn view)

First we create a closure, that triggers our Callback wrapper, which we can use in the `js!` macro:
```rust
impl Renderable<ReactCounter> for ReactCounter {
    fn view(&self) -> Html<Self> {
        let orig_callback = self.react_counter_cb.clone();
        let callback = move || orig_callback.emit(());
        // ...
    }
}
```

We prepare a label with the counter that we will then pass to the React component as a prop:

```rust
impl Renderable<ReactCounter> for ReactCounter {
    fn view(&self) -> Html<Self> {
        // ...
        let label = format!(
            "Native count: {} - React count: {}",
            self.props.native_counter, self.react_counter
        );
        // ...
    }
}
```

Now we come to the rendering of the React component.

Inside the `js!` macro we first create a React element instance of the `MaterialUI.Chip` component (`MaterialUI.Button` has more complicated props requirements).
As a second argument we pass in the props as an object that contains both our label and the callback which serves as a `onClick` handler.

We then use `ReactDOM.render` to render the React element into the Node we created earlier.

```rust
impl Renderable<ReactCounter> for ReactCounter {
    fn view(&self) -> Html<Self> {
        // ...
        js! {
            let element = React.createElement(MaterialUI.Chip,
                {
                  label: @{label},
                  onClick: () => @{callback}(),
                }
              );
            ReactDOM.render(element, @{self.node.clone()});
        }
        // ...
    }
}
```

Lastly we return the node we are rendering into as a virtual DOM reference from the `view` function, so the Yew renderer knows where to attach it to in the Yew component tree.

```rust
impl Renderable<ReactCounter> for ReactCounter {
    fn view(&self) -> Html<Self> {
        // ...
        yew::virtual_dom::VNode::VRef(self.node.clone())
    }
}
```

Here is a complete view of the `view` function:

```rust
impl Renderable<ReactCounter> for ReactCounter {
    fn view(&self) -> Html<Self> {
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
```

## 🙏 Acknowledgements

**Based on [yew-wasm-pack-template](https://github.com/yewstack/yew-wasm-pack-template)**

