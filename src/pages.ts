import $ from "jquery";
import { Toast } from "bootstrap";

import Component from "./components.ts";

export default class Page {
  components: Component[];

  constructor(components: Component[]) {
    this.components = components;
  }

  async render() {
    const root = $("#root");
    try {
      await this.reload();
      root.empty();
      const content = this.components
        .map((component) => component.render())
        .join();
      root.append(content);
    } catch (err) {
      if ($("#page-render-error-container").length === 0) {
        const toast = $(`
          <div id="page-render-error-container" class="toast-container position-fixed bottom-0 end-0 p-3">
            <div id="page-render-error" class="toast" role="alert" aria-live="assertive" aria-atomic="true">
              <div class="toast-header">
                <img src="..." class="rounded me-2" alt="...">
                <strong class="me-auto">Error</strong>
                <button type="button" class="btn-close" data-bs-dismiss="toast" aria-label="Close"></button>
              </div>
              <div class="toast-body">
                ${JSON.stringify(err)}
              </div>
            </div>
          </div>
        `);
        root.append(toast);
      }
      Toast.getOrCreateInstance($("#page-render-error")[0]).show();
    }
  }

  reload(): Promise<any> {
    return Promise.all(this.components.map((component) => component.reload()));
  }
}
