import $ from "jquery";

import Component from "../components.ts";

export default class BasicContainer extends Component {
  children: Component[];

  constructor(children: Component[]) {
    const renderedChildren = children.map((child) => child.render()).join();
    const content = $(`
      <div class="container-fluid">${renderedChildren}</div>
    `);
    super(content);

    this.children = children;
  }

  reload(): Promise<any> {
    return Promise.all(this.children.map((child) => child.reload()));
  }
}
