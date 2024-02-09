import $ from "jquery";

import Component from "../components.ts";
import {
  AppConfigService,
  AppConfigBrief,
} from "../services/api/app_config.ts";
import i18next from "i18next";

export default class AppConfig extends Component {
  appConfigService: AppConfigService;
  data: AppConfigBrief | null;

  constructor(appConfigService: AppConfigService) {
    const content = $(`
      <div id="app-config" class="container-fluid"></div>
    `);
    super(content);

    this.appConfigService = appConfigService;
    this.data = null;
  }

  async reload(): Promise<any> {
    this.data = (await this.appConfigService.index()).body;
    this.content = $(`
      <div id="app-config" class="container-fluid">
        <h1>${i18next.t("components.app_config.title")}</h1>
      </div>
    `);
  }
}
