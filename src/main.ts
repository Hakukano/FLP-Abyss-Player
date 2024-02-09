import "./scss/styles.scss";

import AppConfig from "./pages/app_config.ts";

window.addEventListener("DOMContentLoaded", async () => {
  globalThis.page = new AppConfig();
  await globalThis.page.render();
});
