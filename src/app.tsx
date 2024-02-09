import React from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom";

import { AppConfigService } from "./services/api/app_config.ts";
import Layout from "./pages/layout.tsx";
import Config from "./pages/configs.tsx";

interface AppProps extends React.HTMLAttributes<HTMLElement> {
  appConfigService: AppConfigService;
}

function App(props: AppProps) {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<Config />} />
          <Route path="config" element={<Config />} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
