import { BrowserRouter, Route, Routes } from "react-router-dom";

import { AppConfigService } from "./services/api/app_config.ts";
import Layout from "./pages/layout.tsx";
import Config from "./pages/configs.tsx";

interface Props {
  appConfigService: AppConfigService;
}

function App(props: Props) {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route
            index
            element={<Config appConfigService={props.appConfigService} />}
          ></Route>
          <Route
            path="config"
            element={<Config appConfigService={props.appConfigService} />}
          ></Route>
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
