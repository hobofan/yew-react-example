export function render_material_ui_chip(node, label, onClick) {
  let element = React.createElement(MaterialUI.Chip,
    {
      label,
      onClick,
    }
  );
  ReactDOM.render(element, node);
}
