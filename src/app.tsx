import { BrowserRouter, Route, Routes } from "react-router-dom";

import Welcome from "./pages/welcome.tsx";
import { ApiServices } from "./services/api.ts";

interface Props {
  apiServices: ApiServices;
}

function App(props: Props) {
  return (
    <BrowserRouter>
      <Routes>
        <Route
          path="/"
          element={<Welcome apiServices={props.apiServices} />}
        ></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
