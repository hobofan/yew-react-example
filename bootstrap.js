import("./pkg").then(module => {
  module.run_app();

  class MyComponent extends React.Component {
    constructor(props) {
      super(props);
      console.log('constructor')
      // this.myRef = React.createRef();
      this.myElement = null;
      this.updateFn = null;
      this.setRef = (element) => {
        this.myElement = element;
        // console.log('ref', element);
        this.updateFn = module.render_react_button(element, this.props.text);
      }
    }

    componentWillReceiveProps (nextProps) {
      this.updateFn(this.myElement, nextProps.text);
      // module.render_react_button(this.myElement, nextProps.text);
    }

    render() {
      return React.createElement('div', { ref: this.setRef });
    }
  }
  ReactDOM.render(React.createElement(MyComponent, { text: 'abc' }), document.getElementById('container'));
  ReactDOM.render(React.createElement(MyComponent, { text: 'def' }), document.getElementById('container'));
  // console.log(module.render_react_button);
});
