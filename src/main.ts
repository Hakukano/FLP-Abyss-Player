import "./scss/styles.scss";

import $ from "jquery";

import render from "./components/app_config.ts";

window.addEventListener("DOMContentLoaded", async () => {
  await render($("#root"));
});
