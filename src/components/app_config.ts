import $ from "jquery";

import Component from "../components.ts";
import appConfig, { AppConfigBrief } from "../services/api/app_config.ts";

export default class AppConfig extends Component {
  data: AppConfigBrief | null;

  constructor() {
    const content = $(`
      <div id="app-config" class="container-fluid"></div>
    `);
    super(content);

    this.data = null;
  }

  async reload(): Promise<any> {
    this.data = null;
    try {
      this.data = (await appConfig.index()).body;
    } catch (err: any) {
      alert(err.body);
    }
  }
}
