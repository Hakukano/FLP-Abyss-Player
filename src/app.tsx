import { BrowserRouter, Route, Routes } from "react-router-dom";

import { AppConfigService } from "./services/api/app_config.ts";
import Welcome from "./pages/welcome.tsx";

interface Props {
  appConfigService: AppConfigService;
}

function App(props: Props) {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<Welcome appConfigService={props.appConfigService} />}
        ></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
