import "./scss/styles.scss";

import Configs from "./pages/configs";
import { initialize as initializeAppConfigService } from "./services/api/app_config.ts";
import { initialize as initializeLocaleService } from "./services/locale.ts";

window.addEventListener("DOMContentLoaded", async () => {
  globalThis.appConfigService = initializeAppConfigService();
  globalThis.localeService = await initializeLocaleService(
    globalThis.appConfigService,
  );
  globalThis.page = new Configs(globalThis.appConfigService);
  await globalThis.page.render();
});
